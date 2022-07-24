use super::*;

use crate::indent::{indent_all_by};
use std::fmt::{Formatter, Write};


#[derive(Debug)]
pub enum Statement {
    Export(Export),
}

impl Print for Statement {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        match self {
            Statement::Export(Export::TaggedUnion(tu)) => tu.print(x),
            Statement::Export(Export::Object(ob)) => ob.print(x),
        }
    }
}

#[derive(Debug)]
pub enum Export {
    TaggedUnion(TaggedUnion),
    Object(Object),
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

impl Print for Field {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let ty_string = self.ty.as_string()?;
        write!(x, "{}: {}", self.ident, ty_string)
    }
}

#[derive(Debug)]
pub enum Ty {
    ZodNumber,
    ZodString,
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
        self.print(&mut as_zod)?;
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
            Ty::Reference(raw_ref) => raw_ref.to_string(),
            Ty::Seq(inner) => format!("z.array({})", inner.as_string().expect("local type")),
            Ty::Optional(inner) => format!(
                "{}.optional()",
                inner.as_string().expect("local inner optional type")
            ),
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

#[derive(Debug)]
pub struct Object {
    pub ident: String,
    pub fields: Vec<Field>,
}

impl Print for Object {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln(format!("export const {} = z", self.ident))?;
        printer.indent();
        printer.writeln("z.object({")?;
        printer.indent();
        for field in &self.fields {
            printer.line(field.as_string()?);
        }
        printer.join_lines(',')?;
        printer.dedent();
        printer.writeln("})")?;
        write!(x, "{}", printer.dump())
    }
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
        let mut printer = Printer::new();
        printer.writeln(format!("export const {} = z", self.ident))?;
        printer.indent();
        printer.writeln(format!(".discriminatedUnion({}, [", quote(&self.tag)))?;
        printer.indent();
        for x in &self.variants {
            printer.writeln("z.object({")?;
            printer.indent();
            printer.line(format!("{}: z.literal({})", self.tag, quote(&x.ident)));
            match &x.fields {
                TaggedUnionFields::Unit => {}
                TaggedUnionFields::Fields(fields) => {
                    for field in fields {
                        printer.line(field.as_string()?);
                    }
                }
            }
            printer.join_lines(',')?;
            printer.dedent();
            printer.writeln("}),")?;
        }
        printer.dedent();
        printer.writeln("]);")?;

        write!(x, "{}", printer.dump())
    }
}

struct Printer {
    lines: Vec<String>,
    buffer: String,
    curr: usize,
    size: usize,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            buffer: String::new(),
            curr: 0,
            size: 2,
        }
    }
    pub fn indent(&mut self) {
        self.curr += self.size
    }
    pub fn dedent(&mut self) {
        if self.curr >= self.size {
            self.curr -= self.size
        }
    }
    pub fn writeln<A: AsRef<str>>(&mut self, p: A) -> Result<(), std::fmt::Error> {
        writeln!(self.buffer, "{}", indent_all_by(self.curr, p.as_ref()))
    }
    pub fn line(&mut self, line: impl Into<String>) {
        self.lines.push(indent_all_by(self.curr, line.into()));
    }
    pub fn join_lines(&mut self, join_char: char) -> Result<(), std::fmt::Error> {
        if self.lines.is_empty() {
            return Ok(());
        }
        let indented = self
            .lines
            .iter()
            .map(|l| format!("{}{}", l, join_char))
            .collect::<Vec<_>>()
            .join("\n");
        writeln!(self.buffer, "{}", indented)?;
        self.lines.drain(..);
        Ok(())
    }
    // move ownership
    pub fn dump(self) -> String {
        self.buffer
    }
}

#[test]
fn test_printer() -> Result<(), std::fmt::Error> {
    let mut printer = Printer::new();
    printer.writeln("export const obj = z")?;
    printer.indent();
    printer.writeln(".object({")?;
    printer.indent();
    printer.line("age: z.number()");
    printer.join_lines(',')?;
    printer.join_lines(',')?;
    printer.dedent();
    printer.writeln("})")?;
    let output = printer.dump();
    println!("|{}|", output);
    Ok(())
}
