use super::*;

use crate::printer::{Print, Printer};
use crate::types::import::Import;
use crate::types::object::Object;
use crate::types::tagged_union::TaggedUnion;
use crate::types::union::Union;
use crate::types::zod_enum::Enum;
use std::fmt::Write;

#[derive(Debug)]
pub enum Statement {
    Export(Item),
}

#[derive(Debug)]
pub struct StatementList(pub Vec<Statement>);

impl Print for Statement {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        let (ident, inner) = match self {
            Statement::Export(Item::TaggedUnion(tu)) => (&tu.ident, tu.as_string()?),
            Statement::Export(Item::Object(ob)) => (&ob.ident, ob.as_string()?),
            Statement::Export(Item::Enum(en)) => (&en.ident, en.as_string()?),
            Statement::Export(Item::Lit(lit)) => (&lit.lit, lit.as_string()?),
            Statement::Export(Item::Union(union)) => (&union.ident, union.as_string()?),
        };
        printer.writeln(format!("export const {} =", ident))?;
        printer.indent();
        printer.write(inner)?;
        write!(x, "{}", printer.dump())
    }
}

#[derive(Debug)]
pub enum Item {
    Lit(Literal),
    Enum(Enum),
    Union(Union),
    TaggedUnion(TaggedUnion),
    Object(Object),
}

impl Print for Item {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        match self {
            Item::Lit(lit) => lit.print(x),
            Item::Enum(eenum) => eenum.print(x),
            Item::TaggedUnion(tu) => tu.print(x),
            Item::Object(obj) => obj.print(x),
            Item::Union(uni) => uni.print(x),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub statements: Vec<Statement>,
}

impl Print for Vec<Statement> {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        for statement in self {
            statement.print(x)?;
        }
        Ok(())
    }
}

impl Print for Program {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        self.imports.print(x)?;
        self.statements.print(x)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub ident: String,
    pub ty: Ty,
}

impl Field {
    pub fn new(ident: impl Into<String>, ty: Ty) -> Self {
        Self {
            ident: ident.into(),
            ty,
        }
    }
    pub fn from_syn_field(field: &syn::Field) -> Option<Self> {
        match (&field.ident, as_ty(&field.ty).ok()) {
            (Some(ident), Some(ty)) => Some(Self::new(ident.to_string(), ty)),
            _ => None,
        }
    }
}

impl Print for Field {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let ty_string = self.ty.as_string()?;
        write!(x, "{}: {}", self.ident, ty_string)
    }
}

#[derive(Debug)]
pub struct Literal {
    pub lit: String,
}

impl Print for Literal {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        write!(x, "z.literal({})", quote(&self.lit))
    }
}
