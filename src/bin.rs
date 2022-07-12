#[derive(Debug)]
pub struct Person {
    pub gender: Gender,
}

#[serde_zod::my_attribute(1, 2)]
#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Gender {
    Male,
    Female,
}

fn main() {
    // let _p = Person { name: "shane".into() };
    // let _p = Person::default();
}
