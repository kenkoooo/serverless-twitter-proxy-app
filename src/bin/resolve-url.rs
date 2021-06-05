use anyhow::Context as _;
use lambda_http::{handler, lambda_runtime, Context, IntoResponse, RequestExt};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, UpdateItemInput};
use serde::Deserialize;
use serde_json::json;
use serverless_twitter_proxy_app::TwitterStatus;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Request {
    status_id: String,
}

async fn func(request: lambda_http::Request, _: Context) -> Result<impl IntoResponse, Error> {
    let body = request.payload::<Request>()?;
    let request = body.context("No body")?;

    let data = TwitterStatus {
        status_id: request.status_id,
        data: "Yes!",
    };
    let dynamodb_client = DynamoDbClient::new(Region::ApNortheast1);
    dynamodb_client
        .update_item(data.to_update_item_input()?)
        .await?;
    Ok(json!({"message":"done"}))
}
