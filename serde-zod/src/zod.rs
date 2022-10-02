use super::*;
use std::collections::HashSet;

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

#[test]
fn test_zod_union() {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    enum BlockingState {
        Blocked {},
        Allowed { reason: String },
    }
    #[derive(serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    struct DetectedRequest {
        state: BlockingState,
    }

    // let dt = DetectedRequest {
    //     state: BlockingState::Allowed {
    //         reason: String::from("haha"),
    //     },
    // };
    // let dt2 = DetectedRequest {
    //     state: BlockingState::Blocked {},
    // };
    // let json_1 = serde_json::to_string_pretty(&dt).expect("un");
    // let json_2 = serde_json::to_string_pretty(&dt2).expect("un");
    // println!("{}", json_1);
    // println!("{}", json_2);

    let mut s1 = HashSet::new();
    s1.insert(1);
    s1.insert(2);

    let mut s2 = HashSet::new();
    s2.insert(1);
    s2.insert(2);

    let s3: HashSet<&i32> = s1.intersection(&s2).collect();
    println!("{:?}", s3);
}
