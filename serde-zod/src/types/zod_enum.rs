use crate::printer::{Print, Printer};
use crate::Context;
use std::fmt::Write;

#[derive(Debug)]
pub struct Enum {
    pub ident: String,
    pub variants: Vec<EnumUnitVariant>,
}

impl Enum {
    pub fn new(ident: impl Into<String>) -> Self {
        Self {
            ident: ident.into(),
            variants: vec![],
        }
    }
    pub fn add_variants(&mut self, variants: Vec<EnumUnitVariant>) {
        self.variants.extend(variants);
    }
}

#[derive(Debug)]
pub struct EnumUnitVariant {
    pub ident: String,
}

impl Print for Enum {
    fn print(&self, x: &mut String, _ctx: &Context) -> Result<(), std::fmt::Error> {
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
