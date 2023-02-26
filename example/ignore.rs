extern crate serde_filter;
extern crate serde_json;
use serde_filter::prelude::*;
use serde_json::json;

fn main() {
    let json = json!(
        {
            "explanation": "test",
            "media_type": "test",
            "hdurl": "test",
            "service_version": "test",
            "code": 200,
            "msg": "test"
        }
    );
    let values = filter::<Ignore>(json, &Ignore::new(vec!["explanation", "media_type"]));
    if let Ok(trimmed) = values {
        println!("TRIMMED: {:#?}", trimmed);
        assert!(trimmed.get("explanation").is_none());
        assert!(trimmed.get("media_type").is_none());
        assert!(trimmed.get("hdurl").is_some());
    } else {
        panic!();
    }

    let json = json!({
        "2020-01-01": {
            "explanation": "test_1",
            "media_type": "test",
            "hdurl": "test",
            "service_version": "test",
            "code": 200,
            "msg": "test"
        },
        "Object": {
            "2023-01-11": {
                "Object": {
                    "explanation": "test_3",
                    "media_type": "test",
                    "hdurl": "test",
                    "service_version": "test",
                    "code": 200,
                    "msg": "test"
                },
                "explanation": "test_2",
            }
        },
        "explanation": "test_0"
    });

    let ignore = Ignore::new(vec!["explanation"]);
    let trimmed = filter::<Ignore>(json, &ignore).unwrap();
    println!("TRIMMED: {:#?}", trimmed);
    assert!(trimmed["2020-01-01"].get("explanation").is_none(), "test_1");
    assert!(
        trimmed["Object"]["2023-01-11"]["Object"]
            .get("explanation")
            .is_none(),
        "test_3"
    );
    assert!(
        trimmed["Object"]["2023-01-11"].get("explanation").is_none(),
        "test_2"
    );
    assert!(trimmed.get("explanation").is_none(), "test_0");
}
