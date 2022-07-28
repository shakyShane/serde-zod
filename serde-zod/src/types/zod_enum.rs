use crate::printer::{Print, Printer};
use std::fmt::Write;

#[derive(Debug)]
pub struct Enum {
    pub ident: String,
    pub variants: Vec<EnumUnitVariant>,
}

#[derive(Debug)]
pub struct EnumUnitVariant {
    pub ident: String,
}

impl Print for Enum {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln("z.enum([")?;
        printer.indent();
        for x in &self.variants {
            printer.line(&crate::quote(&x.ident));
        }
        printer.join_lines(',')?;
        printer.dedent();
        printer.writeln("])")?;
        write!(x, "{}", printer.dump())
    }
}
