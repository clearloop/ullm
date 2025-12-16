//! Tests for the response module

use ullm_core::Response;

const RESPONSE_JSON: &str = include_str!("../templates/response.json");

#[test]
fn parse_response() {
    let _response: Response = serde_json::from_str(RESPONSE_JSON).unwrap();
}
