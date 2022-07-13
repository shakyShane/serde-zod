use std::fs;

#[derive(Debug)]
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
    EndedPrematurely,
}

fn main() {
    let lines = vec![
        Status::print_imports(),
        TimerResult::print_zod(),
        Status::print_zod(),
    ];
    fs::write("./app/types.ts", lines.join("\n\n")).expect("can write");
}
