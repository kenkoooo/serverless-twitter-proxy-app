use anyhow::Result;
use rusoto_dynamodb::{AttributeValue, PutItemInput, UpdateItemInput};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

const TABLE_NAME: &str = "TwitterStatuses";
const KEY_STATUS_ID: &str = "status_id";
const KEY_CALL_COUNT: &str = "call_count";
const KEY_RETRIEVED_DATA: &str = "retrieved_data";

pub struct TwitterStatus<T> {
    pub status_id: String,
    pub data: T,
}

impl<T: Serialize> TwitterStatus<T> {
    pub fn to_update_item_input(&self) -> Result<UpdateItemInput> {
        let mut key = HashMap::new();
        key.insert(
            KEY_STATUS_ID.to_string(),
            AttributeValue {
                s: Some(self.status_id.to_string()),
                ..Default::default()
            },
        );

        let update_expression = format!(
            r"
        ADD {call_count} :incr
        SET {data} = :data
        ",
            call_count = KEY_CALL_COUNT,
            data = KEY_RETRIEVED_DATA
        );

        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values.insert(
            ":incr".to_string(),
            AttributeValue {
                n: Some(1.to_string()),
                ..Default::default()
            },
        );
        expression_attribute_values.insert(
            ":data".to_string(),
            AttributeValue {
                s: Some(serde_json::to_string(&self.data)?),
                ..Default::default()
            },
        );

        Ok(UpdateItemInput {
            table_name: TABLE_NAME.to_string(),
            key,
            update_expression: Some(update_expression),
            expression_attribute_values: Some(expression_attribute_values),
            ..Default::default()
        })
    }
}
