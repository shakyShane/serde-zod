use crate::types::object::InlineObject;
use crate::{Context, Print};
use std::fmt::Formatter;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub enum Ty {
    ZodNumber,
    ZodString,
    InlineObject(InlineObject),
    Reference(String),
    Seq(Box<Ty>),
    Optional(Box<Ty>),
}

impl Ty {
    pub fn seq(ty: Ty) -> Self {
        Self::Seq(Box::new(ty))
    }
    pub fn optional(ty: Ty) -> Self {
        Self::Optional(Box::new(ty))
    }
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut as_zod = String::new();
        let ctx = Context::default();
        self.print(&mut as_zod, &ctx)?;
        let named: String = match self {
            Ty::ZodNumber => "Ty::ZodNumber".to_string(),
            Ty::ZodString => "Ty::ZodString".to_string(),
            Ty::Reference(_) => "Ty::Reference".to_string(),
            Ty::Seq(inner) => {
                format!("Ty::Seq({})", inner)
            }
            Ty::Optional(inner) => {
                format!("Ty::Optional({})", inner)
            }
            Ty::InlineObject(_) => "Ty::InlineObject(..)".to_string(),
        };
        writeln!(f, "{}", named)?;
        writeln!(f, "\t{}", as_zod)
    }
}

impl Print for Ty {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        let res = match self {
            Ty::ZodNumber => "z.number()".to_string(),
            Ty::ZodString => "z.string()".to_string(),
            Ty::Reference(raw_ref) => raw_ref.to_string(),
            Ty::Seq(inner) => format!("z.array({})", inner.as_string(ctx).expect("local type")),
            Ty::Optional(inner) => format!(
                "{}.optional()",
                inner.as_string(ctx).expect("local inner optional type")
            ),
            Ty::InlineObject(fields) => fields.as_string(ctx)?,
        };
        write!(x, "{}", res)
    }
}
