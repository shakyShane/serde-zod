use super::*;

use std::fmt::{Formatter, Write};

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
    Fields(Vec<Field>),
}

#[derive(Debug)]
pub struct Field {
    pub ident: String,
    pub ty: Ty,
}

#[derive(Debug)]
pub enum Ty {
    ZodNumber,
    ZodString,
    Reference(String),
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut as_zod = String::new();
        self.print(&mut as_zod)?;
        let named = match self {
            Ty::ZodNumber => "Ty::ZodNumber",
            Ty::ZodString => "Ty::ZodString",
            Ty::Reference(_) => "Ty::Reference",
        };
        writeln!(f, "{}", named)?;
        writeln!(f, "\t{}", as_zod)
    }
}

impl Print for Ty {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let res = match self {
            Ty::ZodNumber => "z.number()",
            Ty::ZodString => "z.string()",
            Ty::Reference(raw_ref) => raw_ref,
        };
        write!(x, "{}", res)
    }
}

#[derive(Debug)]
pub struct TaggedUnion {
    pub ident: String,
    pub tag: String,
    pub variants: Vec<TaggedUnionVariant>,
}

pub trait Print {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error>;
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.print(&mut s)?;
        Ok(s)
    }
}

impl Print for TaggedUnion {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut lines = vec![];
        lines.push(format!("export const {} = z", self.ident));
        lines.push(format!(r#"  .discriminatedUnion("{}", ["#, self.tag));
        for x in &self.variants {
            lines.push(format!("    z.object({{"));
            lines.push(format!(
                "      {}: z.literal({}),",
                self.tag,
                quote(&x.ident)
            ));

            // todo: "push other fields"

            match &x.fields {
                TaggedUnionFields::Unit => {}
                TaggedUnionFields::Fields(fields) => {
                    for field in fields {
                        // println!("{}={}", field.ident, field.ty);
                        let ty_string = field.ty.as_string()?;
                        lines.push(format!("      {}: {},", field.ident, ty_string));
                    }
                }
            }

            lines.push(format!("    }}),"));
        }
        lines.push(format!(r#"  ]);"#));

        write!(x, "{}", lines.join("\n"))
    }
}
