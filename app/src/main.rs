use std::fs;

#[derive(Debug)]
pub struct Person {
    pub gender: Gender,
}

#[serde_zod::my_attribute]
#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Gender {
    Male,
    Female,
}

fn main() {
    let lines = vec![Gender::print_imports(), Gender::print_zod()];
    fs::write("./app/types.ts", lines.join("\n")).expect("can write");
}
