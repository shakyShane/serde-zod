use super::*;

use std::fmt::Write;

#[derive(Debug)]
pub enum Statement {
    Export(Export),
}

impl Print for Statement {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        match self {
            Statement::Export(Export::TaggedUnion(tu)) => tu.print(x),
        }
    }
}

#[derive(Debug)]
pub enum Export {
    TaggedUnion(TaggedUnion),
}

#[derive(Debug)]
pub struct Import {
    pub ident: String,
    pub path: String,
}

impl Print for Import {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        writeln!(x, "import {} from {};", self.ident, quote(&self.path))
    }
}

#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub statements: Vec<Statement>,
}

impl Print for Vec<Import> {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        for import in self {
            import.print(x)?;
        }
        Ok(())
    }
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

#[derive(Debug)]
pub struct TaggedUnionVariant {
    pub ident: String,
    pub fields: TaggedUnionFields,
}

#[derive(Debug)]
pub enum TaggedUnionFields {
    Unit,
}

#[derive(Debug)]
pub struct TaggedUnion {
    pub ident: String,
    pub tag: String,
    pub variants: Vec<TaggedUnionVariant>,
}

pub trait Print {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error>;
}

impl Print for TaggedUnion {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut lines = vec![];
        lines.push(format!("export const {} = z", self.ident));
        lines.push(format!(r#"  .discriminatedUnion("{}", ["#, self.tag));
        for x in &self.variants {
            lines.push(format!("    z.object({{"));
            lines.push(format!(
                "      {}: z.literal({})",
                self.tag,
                quote(&x.ident)
            ));
            // todo: "push other fields"
            lines.push(format!("    }}),"));
        }
        lines.push(format!(r#"  ]);"#));

        write!(x, "{}", lines.join("\n"))
    }
}
