use anyhow::Context as _;
use lambda_http::{handler, lambda_runtime, Context, IntoResponse, RequestExt};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, UpdateItemInput};
use serde::Deserialize;
use serde_json::json;
use serverless_twitter_proxy_app::dynamodb::{DataCache, SingleAccess};
use std::convert::TryInto;

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
    let dynamodb_client = DynamoDbClient::new(Region::ApNortheast1);

    let access = SingleAccess {
        status_id: request.status_id.clone(),
    };
    dynamodb_client.update_item(access.into()).await?;

    let data_cache = DataCache {
        status_id: request.status_id.clone(),
        data: "Yes!",
    };
    dynamodb_client.put_item(data_cache.try_into()?).await?;

    Ok(json!({"message":"done"}))
}
