extern crate serde_json;

use rocket;
use rocket::testing::MockRequest;
use rocket::http::Method::*;
use rocket::http::{Status, ContentType};


#[test]
fn post_kiss() {
    let rocket = rocket();

    let mut precond_req = MockRequest::new(Post, "/kiss")
        .header(ContentType::JSON)
        .body(precond_json);
    let mut response = precond_req.dispatch_with(&rocket);
    let body_string = response.body().unwrap().into_string().unwrap();
    let body: Value = serde_json::from_str(body_string.as_str()).ok().unwrap();

    assert_eq!(&body["status_code"].as_str().unwrap(), &"200");
}
