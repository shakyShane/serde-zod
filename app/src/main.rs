mod real;

use crate::real::{AllowReason, BlockingState, DetectedRequest};

use std::fs;

#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Control {
    Stop,
    Toggle,
}

#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Status {
    Start { elapsed: u64, rem: u64 },
    Tick { elapsed: u64, rem: u64 },
    End { result: TimerResult },
}

#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum TimerResult {
    Ended,
    EndedPrematurely { after: u8 },
    Other { items: Vec<Vec<Test>> },
    WithOptional { control: Option<Control> },
}

#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Test {
    One,
    Two,
}

/// -----------------------------

fn main() {
    let lines = vec![
        DetectedRequest::print_imports(),
        AllowReason::print_zod(),
        BlockingState::print_zod(),
        DetectedRequest::print_zod(),
        // Control::print_zod(),
        // TimerResult::print_zod(),
        // Status::print_zod(),
    ];
    fs::write("./app/types.ts", lines.join("\n\n")).expect("can write");
}

#[test]
fn test_tagged_union() -> Result<(), serde_json::Error> {
    let s = Status::End {
        result: TimerResult::EndedPrematurely { after: 2 },
    };
    let json = serde_json::to_string_pretty(&s)?;
    println!("{}", json);
    Ok(())
}

#[test]
fn test_untagged() -> Result<(), serde_json::Error> {
    #[derive(Debug, Clone, serde::Serialize)]
    enum Count {
        One { count: u8 },
        Two,
    }
    let json = serde_json::to_string_pretty(&Count::One { count: 7 })?;
    let expected = r#"{
  "One": {
    "count": 7
  }
}"#;
    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_tagged() -> Result<(), serde_json::Error> {
    #[derive(Debug, Clone, serde::Serialize)]
    #[serde(tag = "kind")]
    enum Count {
        One { count: u8 },
        Two,
    }
    let json = serde_json::to_string_pretty(&Count::One { count: 7 })?;
    let expected = r#"{
  "kind": "One",
  "count": 7
}"#;
    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_tagged_struct() -> Result<(), serde_json::Error> {
    #[derive(Debug, Clone, serde::Serialize)]
    #[serde(tag = "kind")]
    struct Count {
        count: u8,
    }
    let json = serde_json::to_string_pretty(&Count { count: 7 })?;
    let expected = r#"{
  "kind": "Count",
  "count": 7
}"#;
    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_untagged_struct() -> Result<(), serde_json::Error> {
    #[derive(Debug, Clone, serde::Serialize)]
    struct Count {
        count: u8,
    }
    let json = serde_json::to_string_pretty(&Count { count: 7 })?;
    let expected = r#"{
  "count": 7
}"#;
    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_optional_fields() -> Result<(), serde_json::Error> {
    #[derive(Debug, Clone, serde::Serialize)]
    struct Count {
        count: Option<u8>,
    }
    let json = serde_json::to_string_pretty(&Count { count: None })?;
    let expected = r#"{
  "count": 7
}"#;
    assert_eq!(json, expected);
    Ok(())
}
