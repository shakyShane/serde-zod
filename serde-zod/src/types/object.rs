use crate::printer::Printer;
use crate::{Field, Print};
use std::fmt::Write;

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
