use chrono::{FixedOffset, Utc};
use rusoto_dynamodb::{AttributeValue, UpdateItemInput};
use std::collections::HashMap;

const TABLE_NAME: &str = "TwitterProxyAccessCount";
const KEY_STATUS_ID: &str = "StatusId";
const KEY_DATE: &str = "Date";
const KEY_ACCESS_COUNT: &str = "AccessCount";

pub struct SingleAccess {
    pub status_id: String,
}

impl Into<UpdateItemInput> for SingleAccess {
    fn into(self) -> UpdateItemInput {
        let jst_timezone = FixedOffset::east(9 * 3600);
        let datetime = Utc::now().with_timezone(&jst_timezone);

        let mut key = HashMap::new();
        key.insert(
            KEY_STATUS_ID.to_string(),
            AttributeValue {
                s: Some(self.status_id),
                ..Default::default()
            },
        );
        key.insert(
            KEY_DATE.to_string(),
            AttributeValue {
                s: Some(datetime.format("%Y-%m-%d").to_string()),
                ..Default::default()
            },
        );
        let update_expression =
            format!(r"ADD {access_count} :incr", access_count = KEY_ACCESS_COUNT);
        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values.insert(
            ":incr".to_string(),
            AttributeValue {
                n: Some("1".to_string()),
                ..Default::default()
            },
        );
        UpdateItemInput {
            table_name: TABLE_NAME.to_string(),
            key,
            update_expression: Some(update_expression),
            expression_attribute_values: Some(expression_attribute_values),
            ..Default::default()
        }
    }
}
