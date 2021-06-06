use anyhow::Context as _;
use egg_mode::entities::VideoInfo;
use egg_mode::{KeyPair, Token};
use lambda_http::http::StatusCode;
use lambda_http::{handler, lambda_runtime, Body, Context, IntoResponse, RequestExt, Response};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use serde::Deserialize;
use serverless_twitter_proxy_app::dynamodb::{AccessLogger, VideoInfoCacheClient};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Request {
    status_id: u64,
}

async fn func(request: lambda_http::Request, _: Context) -> Result<Response<Body>, Error> {
    let consumer_key = std::env::var("CONSUMER_KEY")?;
    let consumer_secret = std::env::var("CONSUMER_SECRET")?;
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let access_secret = std::env::var("ACCESS_SECRET")?;
    let token = Token::Access {
        consumer: KeyPair::new(consumer_key, consumer_secret),
        access: KeyPair::new(access_token, access_secret),
    };
    let status_id = request.payload::<Request>()?.context("No body")?.status_id;

    match handle_request(status_id, token).await? {
        Some(video_info) => {
            let value = serde_json::to_value(&video_info)?;
            Ok(value.into_response())
        }
        None => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::Empty)?;
            Ok(response)
        }
    }
}

async fn handle_request(status_id: u64, token: Token) -> anyhow::Result<Option<VideoInfo>> {
    let dynamodb_client = DynamoDbClient::new(Region::ApNortheast1);
    dynamodb_client.log_access(status_id).await?;

    match dynamodb_client.get_cache(status_id).await? {
        Some(video_info) => Ok(Some(video_info)),
        None => {
            let tweet = egg_mode::tweet::show(status_id, &token).await?.response;
            let video_info = tweet
                .extended_entities
                .and_then(|mut e| e.media.pop())
                .and_then(|media| media.video_info);
            if let Some(video_info) = video_info {
                dynamodb_client
                    .save_cache(status_id, video_info.clone())
                    .await?;
                Ok(Some(video_info))
            } else {
                Ok(None)
            }
        }
    }
}
