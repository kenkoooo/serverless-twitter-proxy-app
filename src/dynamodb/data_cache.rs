use anyhow::Result;
use async_trait::async_trait;
use egg_mode::entities::VideoInfo;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use std::collections::HashMap;
use std::convert::TryInto;

const TABLE_NAME: &str = "TwitterProxyVideoInfoCache";
const KEY_STATUS_ID: &str = "StatusId";
const KEY_VIDEO_INFO: &str = "VideoInfo";

struct VideoInfoCache {
    status_id: u64,
    video_info: VideoInfo,
}

impl TryInto<PutItemInput> for VideoInfoCache {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<PutItemInput> {
        let mut item = HashMap::new();
        item.insert(
            KEY_STATUS_ID.to_string(),
            AttributeValue {
                s: Some(self.status_id.to_string()),
                ..Default::default()
            },
        );
        item.insert(
            KEY_VIDEO_INFO.to_string(),
            AttributeValue {
                s: Some(serde_json::to_string(&self.video_info)?),
                ..Default::default()
            },
        );
        Ok(PutItemInput {
            item,
            table_name: TABLE_NAME.to_string(),
            ..Default::default()
        })
    }
}

#[async_trait]
pub trait VideoInfoCacheClient {
    async fn get_cache(&self, status_id: u64) -> Result<Option<VideoInfo>>;
    async fn save_cache(&self, status_id: u64, video_info: VideoInfo) -> Result<()>;
}

#[async_trait]
impl VideoInfoCacheClient for DynamoDbClient {
    async fn get_cache(&self, status_id: u64) -> Result<Option<VideoInfo>> {
        let mut key = HashMap::new();
        key.insert(
            KEY_STATUS_ID.to_string(),
            AttributeValue {
                s: Some(status_id.to_string()),
                ..Default::default()
            },
        );

        let input = GetItemInput {
            table_name: TABLE_NAME.to_string(),
            key,
            ..Default::default()
        };
        let cached_value = self
            .get_item(input)
            .await?
            .item
            .and_then(|mut map| map.remove(KEY_VIDEO_INFO))
            .and_then(|attribute| attribute.s)
            .and_then(|value| serde_json::from_str::<VideoInfo>(&value).ok());
        Ok(cached_value)
    }
    async fn save_cache(&self, status_id: u64, video_info: VideoInfo) -> Result<()> {
        let cache = VideoInfoCache {
            status_id,
            video_info,
        };
        self.put_item(cache.try_into()?).await?;
        Ok(())
    }
}
