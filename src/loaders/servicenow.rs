use crate::loaders::utils::json;
use anyhow::Result;
use serde_json::Value;

pub fn load_incidents(filename: String) -> Result<Value> {
    json::load_and_parse(filename)
}

pub fn load_assignment_groups(filename: String) -> Result<Value> {
    json::load_and_parse(filename)
}
