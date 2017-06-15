extern crate serde_json;
use rocket_contrib::{JSON, Value};
use rocket::State;
use regex::Regex;
use std::collections::HashMap;
use serde_json::Map;

#[post("/kiss", format = "application/json", data = "<data>")]
pub fn postcond(data: JSON<PostcondReq>) -> JSON<Value> {
    JSON(json!({
                "status_code": "200"
    }))
}
