#![deny(warnings)]
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod error;
pub mod ser;

#[test]
fn basic_test() {
    use ser::to_string;

    #[derive(Debug, Serialize)]
    struct Test {
        timestamp: i64,
        v1: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        v2: Option<i32>
    }

    let t = Test {timestamp: 1495468337, v1: "test".to_string(), v2: None};
    assert!(to_string(&t).unwrap() == r#"{"timestamp":1495468337,"_v1":"test"}"#);
}