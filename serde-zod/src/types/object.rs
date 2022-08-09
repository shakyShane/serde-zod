use crate::printer::{Container, Printer};
use crate::{Field, Print};
use std::fmt::Write;

#[derive(Debug)]
pub struct Object {
    pub ident: String,
    pub renamed: Option<String>,
    pub fields: Vec<Field>,

    // prevent creation without public method
    _internal: Marker,
}

#[derive(Debug)]
struct Marker;

impl Object {
    pub fn new(ident: impl Into<String>, fields: Vec<Field>) -> Self {
        Self {
            ident: ident.into(),
            renamed: None,
            fields,
            _internal: Marker,
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
            _internal: Marker,
        }
    }
}

impl Print for Object {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        print_obj(&self.fields, x)
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
