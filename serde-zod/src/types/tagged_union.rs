use crate::printer::Printer;
use crate::union::{UnionVariant, UnionVariantFields};
use crate::{Context, Print};
use std::fmt::Write;

#[derive(Debug)]
pub struct TaggedUnion {
    pub ident: String,
    pub tag: String,
    pub variants: Vec<UnionVariant>,
}

impl TaggedUnion {
    pub fn new(ident: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            ident: ident.into(),
            tag: tag.into(),
            variants: vec![],
        }
    }
    pub fn add_variants(&mut self, variants: Vec<UnionVariant>) {
        self.variants.extend(variants);
    }
}

impl Print for TaggedUnion {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
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
                        printer.line(field.as_string(ctx)?);
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
