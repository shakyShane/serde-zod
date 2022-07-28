use super::*;

use crate::indent::{indent_all_by};
use crate::types::union::Union;
use std::fmt::{Formatter, Write};


#[derive(Debug)]
pub enum Statement {
    Export(Item),
}

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
pub struct EnumVariant {
    pub ident: String,
    pub fields: EnumVariantFields,
}

#[derive(Debug)]
pub struct UnTaggedUnionVariant {
    pub ident: String,
}

#[derive(Debug)]
pub enum EnumVariantFields {
    Unit,
    Named(Vec<Field>),
    Unnamed(Ty),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Ty {
    ZodNumber,
    InlineObject(InlineObject),
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
            Ty::InlineObject(_) => "Ty::InlineObject(..)".to_string(),
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
            Ty::InlineObject(fields) => fields.as_string()?,
        };
        write!(x, "{}", res)
    }
}

#[derive(Debug)]
pub struct TaggedUnion {
    pub ident: String,
    pub tag: String,
    pub variants: Vec<EnumVariant>,
}

impl Print for TaggedUnion {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln(format!("z.discriminatedUnion({}, [", quote(&self.tag)))?;
        printer.indent();
        for x in &self.variants {
            printer.writeln("z.object({")?;
            printer.indent();
            printer.line(format!("{}: z.literal({})", self.tag, quote(&x.ident)));
            match &x.fields {
                EnumVariantFields::Unit => {}
                EnumVariantFields::Named(fields) => {
                    for field in fields {
                        printer.line(field.as_string()?);
                    }
                }
                EnumVariantFields::Unnamed(_) => {
                    // not allowed via serde rules
                }
            }
            printer.join_lines(',')?;
            printer.dedent();
            printer.writeln("}),")?;
        }
        printer.dedent();
        printer.writeln("])")?;

        write!(x, "{}", printer.dump())
    }
}

#[derive(Debug)]
pub struct Enum {
    pub ident: String,
    pub variants: Vec<UnTaggedUnionVariant>,
}

impl Print for Enum {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln("z.enum([")?;
        printer.indent();
        for x in &self.variants {
            printer.line(&quote(&x.ident));
        }
        printer.join_lines(',')?;
        printer.dedent();
        printer.writeln("])")?;
        write!(x, "{}", printer.dump())
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

#[test]
fn test_print_union() -> Result<(), std::fmt::Error> {
    let t = Union {
        ident: String::from("Count"),
        variants: vec![
            EnumVariant {
                ident: "Two".into(),
                fields: EnumVariantFields::Unnamed(Ty::ZodString),
            },
            EnumVariant {
                ident: "TwoOther".into(),
                fields: EnumVariantFields::Unit,
            },
            EnumVariant {
                ident: "TwoOtherReally".into(),
                fields: EnumVariantFields::Named(vec![Field {
                    ident: "named_1".into(),
                    ty: Ty::ZodNumber,
                }]),
            },
            EnumVariant {
                ident: "Three".into(),
                fields: EnumVariantFields::Unnamed(Ty::Optional(Box::new(Ty::ZodString))),
            },
        ],
    };
    let expected = r#"z.union([
  z.object({
    Two: z.string(),
  }),
  z.literal("TwoOther"),
  z.object({
    TwoOtherReally: z.object({
      named_1: z.number(),
    }),
  }),
  z.object({
    Three: z.string().optional(),
  }),
])"#;
    // let litt = Item::Lit(Literal { lit: "Two".into() }).as_string()?;
    let printed = t.as_string()?;
    assert_eq!(expected, printed);
    Ok(())
}

#[derive(Debug)]
pub struct Object {
    pub ident: String,
    pub fields: Vec<Field>,
}

impl Print for Object {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        print_obj(&self.fields, x)
    }
}

#[derive(Debug, Clone)]
pub struct InlineObject {
    pub fields: Vec<Field>,
}

impl Print for InlineObject {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        print_obj(&self.fields, x)
    }
}

fn print_obj(fields: &[Field], target: &mut String) -> Result<(), std::fmt::Error> {
    let mut printer = Printer::new();
    printer.writeln("z.object({")?;
    printer.indent();
    for field in fields {
        printer.line(field.as_string()?);
    }
    printer.join_lines(',')?;
    printer.dedent();
    printer.writeln("})")?;
    write!(target, "{}", printer.dump())
}

pub trait Print {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error>;
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.print(&mut s)?;
        Ok(s)
    }
}

pub struct Printer {
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
    pub fn write<A: AsRef<str>>(&mut self, p: A) -> Result<(), std::fmt::Error> {
        write!(self.buffer, "{}", indent_all_by(self.curr, p.as_ref()))
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
            .map(|l| {
                let last = &l[l.len() - 1..l.len()];
                if last == "\n" {
                    let without = &l[0..l.len() - 1];
                    format!("{}{}", without, join_char)
                } else {
                    format!("{}{}", l, join_char)
                }
                // if let Some('\n') = l.chars().last() {
                // } else {
                // }
            })
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
