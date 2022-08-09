mod real;

use crate::real::{AllowReason, BlockingState, DetectedRequest};
use std::fs;

///
/// This example shows how you would combine multiple items together
/// into a single .ts file
///
fn main() {
    let lines = vec![
        DetectedRequest::print_imports(),
        AllowReason::codegen(),
        BlockingState::codegen(),
        DetectedRequest::codegen(),
        Control::codegen(),
        Test::codegen(),
        TimerResult::codegen(),
        Status::codegen(),
        MixedEnum::codegen(),
        UnitOnlyEnum::codegen(),
        State::codegen(),
    ];
    fs::write("./example/types.ts", lines.join("\n")).expect("can write");
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub enum Control {
    Start { time: u32 },
    Stop,
    Toggle,
}

#[test]
fn test_control() {
    let actual = Control::codegen();
    let expected = r#"export const Control =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Start"),
      time: z.number(),
    }),
    z.object({
      kind: z.literal("Stop"),
    }),
    z.object({
      kind: z.literal("Toggle"),
    }),
  ])
"#;
    assert_eq!(actual, expected);
}

#[serde_zod::codegen]
#[derive(Debug, Clone, serde::Serialize)]
pub enum UnitOnlyEnum {
    Stop,
    Toggle,
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
pub struct State {
    control: UnitOnlyEnum,
}

#[test]
fn test_unit_only_enum() {
    let actual1 = UnitOnlyEnum::codegen();
    let actual2 = State::codegen();
    let joined = vec![actual1, actual2].join("\n");
    let expected = r#"export const UnitOnlyEnum =
  z.enum([
    "Stop",
    "Toggle",
  ])

export const State =
  z.object({
    control: UnitOnlyEnum,
  })
"#;
    assert_eq!(joined, expected);
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub enum Status {
    Start { elapsed: u64, rem: u64 },
    Tick { elapsed: u64, rem: u64 },
    End { result: TimerResult },
}

#[test]
fn test_status() {
    let actual = Status::codegen();
    let expected = r#"export const Status =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Start"),
      elapsed: z.number(),
      rem: z.number(),
    }),
    z.object({
      kind: z.literal("Tick"),
      elapsed: z.number(),
      rem: z.number(),
    }),
    z.object({
      kind: z.literal("End"),
      result: TimerResult,
    }),
  ])
"#;
    assert_eq!(actual, expected);
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub enum TimerResult {
    Ended,
    EndedPrematurely { after: u8 },
    Other { items: Vec<Vec<Test>> },
    WithOptional { control: Option<Control> },
}

#[test]
fn test_timer_result() {
    let actual = TimerResult::codegen();
    let expected = r#"export const TimerResult =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Ended"),
    }),
    z.object({
      kind: z.literal("EndedPrematurely"),
      after: z.number(),
    }),
    z.object({
      kind: z.literal("Other"),
      items: z.array(z.array(Test)),
    }),
    z.object({
      kind: z.literal("WithOptional"),
      control: Control.optional(),
    }),
  ])
"#;
    assert_eq!(actual, expected);
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub enum Test {
    One,
    Two,
}

#[test]
fn test_test() {
    let actual = Test::codegen();
    let expected = r#"export const Test =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("One"),
    }),
    z.object({
      kind: z.literal("Two"),
    }),
  ])
"#;
    assert_eq!(actual, expected);
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
pub enum MixedEnum {
    One,
    Two(String),
    Three { temp: usize },
}

#[test]
fn test_count_2() {
    let actual = MixedEnum::codegen();
    let expected = r#"export const MixedEnum =
  z.union([
    z.literal("One"),
    z.object({
      Two: z.string(),
    }),
    z.object({
      Three: z.object({
        temp: z.number(),
      }),
    }),
  ])"#;
    assert_eq!(actual, expected);
}
