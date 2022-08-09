use crate::printer::{Container, Printer};
use crate::types::field::Field;
use crate::{Context, Print};
use std::fmt::Write;

#[derive(Debug)]
pub struct Object {
    pub ident: String,
    pub renamed: Option<String>,
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Marker;

#[allow(dead_code)]
impl Object {
    pub fn new(ident: impl Into<String>, fields: Vec<Field>) -> Self {
        Self {
            ident: ident.into(),
            renamed: None,
            fields,
        }
    }
    pub fn new_renamed(
        ident: impl Into<String>,
        renamed: Option<String>,
        fields: Vec<Field>,
    ) -> Self {
        Self {
            ident: ident.into(),
            renamed,
            fields,
        }
    }
    pub fn field(&mut self, field: impl Into<Field>) {
        self.fields.push(field.into())
    }
}

impl Print for Object {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        print_obj(&self.fields, x, ctx)
    }
}

impl Container for Object {
    fn display_ident(&self) -> &String {
        self.renamed.as_ref().unwrap_or(&self.ident)
    }
}

#[derive(Debug, Clone)]
pub struct InlineObject {
    pub fields: Vec<Field>,
}

impl Print for InlineObject {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        print_obj(&self.fields, x, ctx)
    }
}

fn print_obj(fields: &[Field], target: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
    let mut printer = Printer::new();
    printer.writeln("z.object({")?;
    printer.indent();
    for field in fields {
        printer.line(field.as_string(ctx)?);
    }
    printer.join_lines(',')?;
    printer.dedent();
    printer.writeln("})")?;
    write!(target, "{}", printer.dump())
}
