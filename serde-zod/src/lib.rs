mod zod;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use quote::quote;
use zod::*;

use crate::zod::Program;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Error, Field, Fields, Meta,
    NestedMeta,
};

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let serde_derive = has_serde_derive(&input_parsed.attrs);
    let serde_attr = serde_attrs(&input_parsed.attrs);

    if !serde_derive {
        return Error::new(
            Span::call_site(),
            "must derive serde::Serialize or serde::Deserialize",
        )
        .to_compile_error()
        .into();
    }

    if serde_attr.is_empty() {
        return Error::new(
            Span::call_site(),
            "must derive serde::Serialize or serde::Deserialize",
        )
        .to_compile_error()
        .into();
    }

    let p = match &input_parsed.data {
        Data::Struct(_) => todo!("Data::Struct"),
        Data::Union(_) => todo!("Data::Union"),
        Data::Enum(e) => process_tagged_enum(&input_parsed.ident, e, "kind"),
    };

    let p = p.expect("unwrap program");
    let mut st = String::new();
    let mut im = String::new();
    p.statements.print(&mut st).expect("printing statements");
    p.imports.print(&mut im).expect("printing imports");

    let tokens = quote! {
        #input_parsed
        impl Gender {
            pub fn print_zod() -> String {
                String::from(#st)
            }
            pub fn print_imports() -> String {
                String::from(#im)
            }
        }
    };

    tokens.into()
}

fn process_tagged_enum(ident: &Ident, e: &DataEnum, tag: &str) -> Result<Program, std::fmt::Error> {
    let mut tu = zod::TaggedUnion {
        ident: ident.to_string(),
        tag: tag.to_string(),
        variants: vec![],
    };

    e.variants.iter().for_each(|vari| {
        // println!("variant ident: {}", vari.ident);
        match &vari.fields {
            Fields::Named(fields_named) => {
                let mut fields: Vec<zod::Field> = vec![];
                for field in &fields_named.named {
                    println!("field {:?}", field.ident);
                    if let Some(ident) = &field.ident {
                        fields.push(zod::Field {
                            ident: ident.to_string(),
                            ty: String::from("String"),
                        })
                    }
                }
                let tuv = zod::TaggedUnionVariant {
                    ident: vari.ident.to_string(),
                    fields: zod::TaggedUnionFields::Fields(fields),
                };
                tu.variants.push(tuv);
            }
            Fields::Unnamed(_) => unreachable!("unamed not yet supported"),
            Fields::Unit => {
                let tuv = zod::TaggedUnionVariant {
                    ident: vari.ident.to_string(),
                    fields: zod::TaggedUnionFields::Unit,
                };
                tu.variants.push(tuv);
            }
        }
    });

    let mut p = Program {
        statements: vec![],
        imports: vec![],
    };
    p.imports.push(zod::Import {
        ident: "z".into(),
        path: "zod".into(),
    });
    p.statements
        .push(Statement::Export(Export::TaggedUnion(tu)));
    Ok(p)
}

fn quote<A: AsRef<str>>(a: &A) -> String {
    format!("\"{}\"", a.as_ref())
}

fn serde_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|att| att.path.get_ident().filter(|v| *v == "serde").is_some())
        .cloned()
        .collect()
}

fn has_serde_derive(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| {
            if let Meta::List(l) = meta {
                return Some(l.nested.into_iter());
            }
            None
        })
        .flatten()
        .any(|x| match x {
            NestedMeta::Meta(Meta::Path(path)) => {
                // first is 'serde'
                let first = path.segments.first().filter(|x| x.ident == "serde");

                // second is "Serialize" or "Deserialize"
                let sub = path
                    .segments
                    .iter()
                    .any(|s| s.ident == "Serialize" || s.ident == "Deserialize");

                matches!((first, sub), (Some(..), true))
            }
            _ => false,
        })
}
