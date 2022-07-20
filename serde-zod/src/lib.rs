mod indent;
mod zod;

extern crate proc_macro;
use indenter;
use indenter::Format;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use std::fmt::Write;

use quote::quote;
use zod::*;

use crate::indent::{indent_all_by, indent_by};
use crate::zod::Program;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Error, Field, Fields,
    GenericArgument, Meta, NestedMeta, PathArguments, Type,
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

    let impl_ident = input_parsed.ident.clone();

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
        impl #impl_ident {
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
                    let ty = as_ty(&field.ty).expect("ty");
                    if let Some(ident) = &field.ident {
                        fields.push(zod::Field {
                            ident: ident.to_string(),
                            ty,
                        })
                    }
                }
                let tuv = zod::TaggedUnionVariant {
                    ident: vari.ident.to_string(),
                    fields: TaggedUnionFields::Fields(fields),
                };
                tu.variants.push(tuv);
            }
            Fields::Unnamed(_) => unreachable!("unamed not yet supported"),
            Fields::Unit => {
                let tuv = zod::TaggedUnionVariant {
                    ident: vari.ident.to_string(),
                    fields: TaggedUnionFields::Unit,
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

fn as_ty(ty: &Type) -> Result<Ty, String> {
    match ty {
        Type::Path(p) => {
            // println!("Type::Path({:?})");

            // is it a raw ident, like 'u8'
            if let Some(ident) = p.path.get_ident() {
                return Ok(rust_ident_to_ty(ident.to_string()));
            }

            for x in &p.path.segments {
                let ident = x.ident.to_string();
                match &x.arguments {
                    PathArguments::None => {
                        println!("none")
                    }
                    PathArguments::AngleBracketed(o) => {
                        for a in &o.args {
                            match a {
                                GenericArgument::Lifetime(_) => {
                                    println!("GenericArgument::Lifetime")
                                }
                                GenericArgument::Type(ty) => {
                                    println!("GenericArgument::Type");
                                    let r = as_ty(ty);
                                    if let Ok(zod_ty) = r {
                                        if ident == "Vec" {
                                            return Ok(Ty::seq(zod_ty));
                                        }
                                    }
                                }
                                GenericArgument::Binding(_) => {
                                    println!("GenericArgument::Binding")
                                }
                                GenericArgument::Constraint(_) => {
                                    println!("GenericArgument::Constraint")
                                }
                                GenericArgument::Const(_) => {
                                    println!("GenericArgument::Const")
                                }
                            }
                        }
                    }
                    PathArguments::Parenthesized(_) => {
                        println!("para")
                    }
                }
            }

            return Err("could not get identifier".into());
        }
        Type::Array(_) => {
            println!("Type::Array");
            Ok(zod::Ty::ZodString)
        }
        // Type::BareFn(_) => {
        //     println!("Type::BareFn")
        // }
        // Type::Group(_) => {
        //     println!("Type::Group")
        // }
        // Type::ImplTrait(_) => {
        //     println!("Type::ImplTrait")
        // }
        // Type::Infer(_) => {
        //     println!("Type::Infer")
        // }
        // Type::Macro(_) => {
        //     println!("Type::Macro")
        // }
        // Type::Never(_) => {
        //     println!("Type::Never")
        // }
        // Type::Paren(_) => {
        //     println!("Type::Paren")
        // }
        // Type::Ptr(_) => {
        //     println!("Type::Ptr")
        // }
        // Type::Reference(_) => {
        //     println!("Type::Reference")
        // }
        // Type::Slice(_) => {
        //     println!("Type::Slice")
        // }
        // Type::TraitObject(_) => {
        //     println!("Type::TraitObject")
        // }
        // Type::Tuple(_) => {
        //     println!("Type::Tuple")
        // }
        // Type::Verbatim(ver) => {
        //     println!("Type::Verbatim")
        // }
        _ => Err(String::from("unknown")),
    }
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

fn rust_ident_to_ty<A: AsRef<str>>(raw_ident: A) -> Ty {
    println!("{}", raw_ident.as_ref());
    match raw_ident.as_ref() {
        "u8" => Ty::ZodNumber,
        "u64" => Ty::ZodNumber,
        "String" => Ty::ZodString,
        ident => Ty::Reference(ident.to_string()),
    }
}

#[test]
fn test_indent() {
    let input = "verify\n\nthis";
    let mut output = String::new();

    let r = indenter::indented(&mut output)
        .with_format(Format::Uniform {
            indentation: "    ",
        })
        .write_str(input)
        .unwrap();

    println!("Before:\n|{}|\n", input);
    println!("After:\n|{}|", output);

    let lines = vec!["z.literal('here')", "z.literal('there')"];
    let joined = lines.join(",\n");
    let width = 4;
    println!("|{}|", indent_all_by(width, joined));
}
