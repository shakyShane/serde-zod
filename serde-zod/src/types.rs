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
