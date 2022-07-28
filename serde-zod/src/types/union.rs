use crate::zod::Printer;
use crate::{EnumVariant, EnumVariantFields, Field, InlineObject, Print, Ty};
use std::fmt::Write;

#[derive(Debug)]
pub struct Union {
    pub ident: String,
    pub variants: Vec<EnumVariant>,
}

impl Print for Union {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln("z.union([")?;
        printer.indent();
        for x in &self.variants {
            match &x.fields {
                EnumVariantFields::Unit => {
                    let as_lit = crate::Literal {
                        lit: x.ident.clone(),
                    };
                    printer.line(as_lit.as_string()?);
                }
                EnumVariantFields::Named(fields) => {
                    let object_key = x.ident.clone();
                    let ident_obj = crate::Object {
                        ident: object_key,
                        fields: vec![Field {
                            ident: x.ident.clone(),
                            ty: Ty::InlineObject(InlineObject {
                                fields: fields.clone(),
                            }),
                        }],
                    };
                    printer.line(&ident_obj.as_string()?);
                }
                EnumVariantFields::Unnamed(ty) => {
                    let object_key = x.ident.clone();
                    let as_obj = crate::Object {
                        ident: object_key,
                        fields: vec![Field {
                            ident: x.ident.clone(),
                            ty: ty.clone(),
                        }],
                    };
                    printer.line(&as_obj.as_string()?);
                }
            }
        }
        printer.join_lines(',')?;
        printer.dedent();
        printer.write("])")?;
        write!(x, "{}", printer.dump())
    }
}
