use super::*;

use crate::printer::{Container, Print, Printer};
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
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        let (ident, inner) = match self {
            Statement::Export(Item::TaggedUnion(tu)) => (&tu.ident, tu.as_string(ctx)?),
            Statement::Export(Item::Object(ob)) => (ob.display_ident(), ob.as_string(ctx)?),
            Statement::Export(Item::Enum(en)) => (&en.ident, en.as_string(ctx)?),
            Statement::Export(Item::Lit(lit)) => (&lit.lit, lit.as_string(ctx)?),
            Statement::Export(Item::Union(union)) => (&union.ident, union.as_string(ctx)?),
        };
        printer.writeln(format!("export const {} =", ident))?;
        printer.indent();
        printer.write(inner)?;
        write!(x, "{}", printer.dump())
    }
}

#[derive(Debug)]
pub enum Item {
    #[allow(dead_code)]
    Lit(Literal),
    Enum(Enum),
    Union(Union),
    TaggedUnion(TaggedUnion),
    Object(Object),
}

impl Print for Item {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        match self {
            Item::Lit(lit) => lit.print(x, ctx),
            Item::Enum(eenum) => eenum.print(x, ctx),
            Item::TaggedUnion(tu) => tu.print(x, ctx),
            Item::Object(obj) => obj.print(x, ctx),
            Item::Union(uni) => uni.print(x, ctx),
        }
    }
}

impl Print for Vec<Statement> {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        for statement in self {
            statement.print(x, ctx)?;
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub statements: Vec<Statement>,
}

impl Print for Program {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        self.imports.print(x, ctx)?;
        self.statements.print(x, ctx)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Literal {
    pub lit: String,
}

impl Print for Literal {
    fn print(&self, x: &mut String, _ctx: &Context) -> Result<(), std::fmt::Error> {
        write!(x, "z.literal({})", quote(&self.lit))
    }
}
