use anyhow::Result;
use rusoto_dynamodb::{AttributeValue, PutItemInput};
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryInto;

const TABLE_NAME: &str = "TwitterProxyDataCache";
const KEY_STATUS_ID: &str = "StatusId";
const KEY_CACHED_DATA: &str = "CachedData";

pub struct DataCache<T> {
    pub status_id: String,
    pub data: T,
}

impl<T: Serialize> TryInto<PutItemInput> for DataCache<T> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<PutItemInput, Self::Error> {
        let mut item = HashMap::new();
        item.insert(
            KEY_STATUS_ID.to_string(),
            AttributeValue {
                s: Some(self.status_id),
                ..Default::default()
            },
        );
        item.insert(
            KEY_CACHED_DATA.to_string(),
            AttributeValue {
                s: Some(serde_json::to_string(&self.data)?),
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
