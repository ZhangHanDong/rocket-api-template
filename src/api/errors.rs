use rocket_contrib::{JSON, Value};

#[error(404)]
pub fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[error(500)]
pub fn internal_error() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Whoops! Looks like we messed up."
    }))
}
