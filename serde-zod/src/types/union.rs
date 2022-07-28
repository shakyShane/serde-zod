use crate::printer::{Print, Printer};
use crate::types::object::InlineObject;
use crate::types::ty::Ty;
use crate::{as_ty, Field};
use std::fmt::Write;

#[derive(Debug)]
pub struct Union {
    pub ident: String,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug)]
pub struct UnionVariant {
    pub ident: String,
    pub fields: UnionVariantFields,
}

impl UnionVariant {
    pub fn from_syn_fields_named(
        ident: impl Into<String>,
        fields_named: &syn::FieldsNamed,
    ) -> Option<Self> {
        let fields = fields_named
            .named
            .iter()
            .filter_map(Field::from_syn_field)
            .collect();
        let tuv = Self {
            ident: ident.into(),
            fields: UnionVariantFields::Named(fields),
        };
        Some(tuv)
    }
    pub fn from_syn_fields_unnamed(
        ident: impl Into<String>,
        unnamed: &syn::FieldsUnnamed,
    ) -> Option<Self> {
        unnamed
            .unnamed
            .first()
            .and_then(|first| as_ty(&first.ty).ok())
            .map(|ty| Self {
                ident: ident.into(),
                fields: UnionVariantFields::Unnamed(ty),
            })
    }
    pub fn from_unit(ident: impl Into<String>) -> Self {
        Self {
            ident: ident.into(),
            fields: UnionVariantFields::Unit,
        }
    }
}

#[derive(Debug)]
pub enum UnionVariantFields {
    Unit,
    Named(Vec<Field>),
    Unnamed(Ty),
}

impl Print for Union {
    fn print(&self, x: &mut String) -> Result<(), std::fmt::Error> {
        let mut printer = Printer::new();
        printer.writeln("z.union([")?;
        printer.indent();
        for x in &self.variants {
            match &x.fields {
                UnionVariantFields::Unit => {
                    let as_lit = crate::Literal {
                        lit: x.ident.clone(),
                    };
                    printer.line(as_lit.as_string()?);
                }
                UnionVariantFields::Named(fields) => {
                    let object_key = x.ident.clone();
                    let ident_obj = crate::types::object::Object {
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
                UnionVariantFields::Unnamed(ty) => {
                    let object_key = x.ident.clone();
                    let as_obj = crate::types::object::Object {
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

#[test]
fn test_print_union() -> Result<(), std::fmt::Error> {
    let t = Union {
        ident: String::from("Count"),
        variants: vec![
            UnionVariant {
                ident: "Two".into(),
                fields: UnionVariantFields::Unnamed(Ty::ZodString),
            },
            UnionVariant {
                ident: "TwoOther".into(),
                fields: UnionVariantFields::Unit,
            },
            UnionVariant {
                ident: "TwoOtherReally".into(),
                fields: UnionVariantFields::Named(vec![Field {
                    ident: "named_1".into(),
                    ty: Ty::ZodNumber,
                }]),
            },
            UnionVariant {
                ident: "Three".into(),
                fields: UnionVariantFields::Unnamed(Ty::Optional(Box::new(Ty::ZodString))),
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
