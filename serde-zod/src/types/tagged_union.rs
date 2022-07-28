use crate::printer::Printer;
use crate::union::{UnionVariant, UnionVariantFields};
use crate::Print;
use std::fmt::Write;

#[derive(Debug)]
pub struct TaggedUnion {
    pub ident: String,
    pub tag: String,
    pub variants: Vec<UnionVariant>,
}

impl Print for TaggedUnion {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln(format!(
            "z.discriminatedUnion({}, [",
            crate::quote(&self.tag)
        ))?;
        printer.indent();
        for x in &self.variants {
            printer.writeln("z.object({")?;
            printer.indent();
            printer.line(format!(
                "{}: z.literal({})",
                self.tag,
                crate::quote(&x.ident)
            ));
            match &x.fields {
                UnionVariantFields::Unit => {}
                UnionVariantFields::Named(fields) => {
                    for field in fields {
                        printer.line(field.as_string()?);
                    }
                }
                UnionVariantFields::Unnamed(_) => {
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
