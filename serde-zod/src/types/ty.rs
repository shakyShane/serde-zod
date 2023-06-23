use crate::types::object::InlineObject;
use crate::Print;
use std::fmt::Formatter;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub enum Ty {
    ZodNumber,
    ZodString,
    ZodBoolean,
    InlineObject(InlineObject),
    Reference(String),
    Seq(Box<Ty>),
    Optional(Box<Ty>),
    Record(Box<Ty>, Box<Ty>),
    Tuple(Vec<Ty>),
}

impl Ty {
    pub fn seq(ty: Ty) -> Self {
        Self::Seq(Box::new(ty))
    }
    pub fn optional(ty: Ty) -> Self {
        Self::Optional(Box::new(ty))
    }
    pub fn record(key: Ty, value: Ty) -> Self {
        Self::Record(Box::new(key), Box::new(value))
    }
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut as_zod = String::new();
        self.print(&mut as_zod)?;
        let named: String = match self {
            Ty::ZodNumber => "Ty::ZodNumber".to_string(),
            Ty::ZodString => "Ty::ZodString".to_string(),
            Ty::ZodBoolean => "Ty::ZodBoolean".to_string(),
            Ty::Reference(_) => "Ty::Reference".to_string(),
            Ty::Seq(inner) => {
                format!("Ty::Seq({})", inner)
            }
            Ty::Optional(inner) => {
                format!("Ty::Optional({})", inner)
            }
            Ty::InlineObject(_) => "Ty::InlineObject(..)".to_string(),
            Ty::Record(key, value) => {
                format!("Ty::Record({key}, {value})")
            }
            Ty::Tuple(i) => format!("Ty::InlineObject([_; {}])", i.len()),
        };
        writeln!(f, "{}", named)?;
        writeln!(f, "\t{}", as_zod)
    }
}

impl Print for Ty {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let res = match self {
            Ty::ZodNumber => "z.number()".to_string(),
            Ty::ZodString => "z.string()".to_string(),
            Ty::ZodBoolean => "z.boolean()".to_string(),
            Ty::Reference(raw_ref) => raw_ref.to_string(),
            Ty::Seq(inner) => format!("z.array({})", inner.as_string().expect("local type")),
            Ty::Optional(inner) => format!(
                "{}.optional()",
                inner.as_string().expect("local inner optional type")
            ),
            Ty::InlineObject(fields) => fields.as_string()?,
            Ty::Record(key, value) => {
                format!(
                    "z.record({}, {})",
                    key.as_string().expect("local type"),
                    value.as_string().expect("local type")
                )
            }
            Ty::Tuple(inner) => {
                let mut collect = Vec::with_capacity(inner.len());
                for i in inner.iter() {
                    collect.push(i.as_string().expect("local type"));
                }
                format!("z.tuple([{}])", collect.join(", "))
            }
        };
        write!(x, "{}", res)
    }
}
