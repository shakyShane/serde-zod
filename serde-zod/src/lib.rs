mod get_serde_rename;
mod indent;
mod printer;
mod types;
mod zod;

extern crate proc_macro;
// use indenter;

use crate::printer::Print;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Index;

use zod::*;

use crate::tagged_union::TaggedUnion;
use crate::types::zod_enum::{Enum, EnumUnitVariant};
use crate::union::UnionVariant;
use crate::zod::Program;

use crate::get_serde_rename::get_serde_rename;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields,
    GenericArgument, Meta, MetaNameValue, NestedMeta, PathArguments, Type,
};
use types::ty::Ty;
use types::{import, object, tagged_union, union};

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn codegen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let serde_derive = has_serde_derive(&input_parsed.attrs);
    let serde_attrs = serde_attrs(&input_parsed.attrs);

    if !serde_derive {
        return Error::new(
            Span::call_site(),
            "must derive serde::Serialize or serde::Deserialize",
        )
        .to_compile_error()
        .into();
    }

    let impl_ident = input_parsed.ident.clone();

    let statements: Result<StatementList, _> = match &input_parsed.data {
        Data::Struct(st) => StatementList::try_from((&input_parsed.ident, st)),
        Data::Union(_) => todo!("Data::Union"),
        Data::Enum(e) => {
            let tag = serde_attrs.get("tag");
            if let Some(tag) = tag {
                StatementList::try_from((EnumKind::Tagged(tag.clone()), &input_parsed.ident, e))
            } else {
                let all_unit = e.variants.iter().all(|v| matches!(&v.fields, Fields::Unit));
                if all_unit {
                    StatementList::try_from((EnumKind::UnitOnly, &input_parsed.ident, e))
                } else {
                    StatementList::try_from((EnumKind::Mixed, &input_parsed.ident, e))
                }
            }
        }
    };

    let statements = match statements {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("{}", e);
            return Error::new(Span::call_site(), "Couldn't create statements")
                .to_compile_error()
                .into();
        }
    };

    let mut p = Program {
        statements: vec![],
        imports: vec![],
    };

    p.imports.push(import::Import {
        ident: "z".into(),
        path: "zod".into(),
    });

    p.statements.extend(statements.0);

    let mut st = String::new();
    let mut im = String::new();

    p.statements.print(&mut st).expect("printing statements");
    p.imports.print(&mut im).expect("printing imports");

    let tokens = quote! {
        #input_parsed
        impl #impl_ident {
            pub fn codegen() -> String {
                String::from(#st)
            }
            pub fn print_imports() -> String {
                String::from(#im)
            }
        }
    };

    tokens.into()
}

fn process_struct(
    ident: &Ident,
    data_struct: &DataStruct,
) -> Result<Vec<Statement>, std::fmt::Error> {
    let mut ob = object::Object {
        ident: ident.to_string(),
        fields: Default::default(),
    };
    for field in &data_struct.fields {
        let ty = as_ty(&field.ty).expect("ty");
        if let Some(ident) = &field.ident {
            let name = get_serde_rename(&field.attrs).unwrap_or(ident.to_string());
            ob.fields.push(zod::Field { ident: name, ty })
        }
    }
    let statements = vec![Statement::Export(Item::Object(ob))];
    Ok(statements)
}

fn process_mixed_enum(ident: &Ident, e: &DataEnum) -> Result<Vec<Statement>, std::fmt::Error> {
    let mut zod_union = union::Union {
        ident: ident.to_string(),
        variants: vec![],
    };
    let variants = extract_variants(e);
    zod_union.variants.extend(variants);
    Ok(vec![Statement::Export(Item::Union(zod_union))])
}

enum EnumKind {
    Tagged(String),
    UnitOnly,
    Mixed,
}

impl TryFrom<(EnumKind, &Ident, &DataEnum)> for StatementList {
    type Error = std::fmt::Error;

    fn try_from((kind, ident, e_enum): (EnumKind, &Ident, &DataEnum)) -> Result<Self, Self::Error> {
        match kind {
            EnumKind::Tagged(tag) => process_tagged_enum(ident, e_enum, &tag),
            EnumKind::UnitOnly => process_unit_only_enum(ident, e_enum),
            EnumKind::Mixed => process_mixed_enum(ident, e_enum),
        }
        .map(StatementList)
    }
}

impl TryFrom<(&Ident, &DataStruct)> for StatementList {
    type Error = std::fmt::Error;

    fn try_from((ident, data_struct): (&Ident, &DataStruct)) -> Result<Self, Self::Error> {
        process_struct(ident, data_struct).map(StatementList)
    }
}

fn process_unit_only_enum(ident: &Ident, e: &DataEnum) -> Result<Vec<Statement>, std::fmt::Error> {
    let mut zod_enum = Enum::new(ident.to_string());
    let variants = e
        .variants
        .iter()
        .filter_map(|variant| match variant.fields {
            Fields::Unit => Some(EnumUnitVariant {
                ident: variant.ident.to_string(),
            }),
            _ => None,
        })
        .collect();
    zod_enum.add_variants(variants);
    let statements = vec![Statement::Export(Item::Enum(zod_enum))];
    Ok(statements)
}

fn process_tagged_enum(
    ident: &Ident,
    e: &DataEnum,
    tag: &str,
) -> Result<Vec<Statement>, std::fmt::Error> {
    let mut tagged_union = TaggedUnion::new(ident.to_string(), tag);
    tagged_union.add_variants(extract_variants(e));
    let statements = vec![Statement::Export(Item::TaggedUnion(tagged_union))];
    Ok(statements)
}

fn extract_variants(data_enum: &DataEnum) -> Vec<UnionVariant> {
    data_enum
        .variants
        .iter()
        .filter_map(|vari| {
            let ident = vari.ident.to_string();
            match &vari.fields {
                Fields::Named(fields_named) => {
                    UnionVariant::from_syn_fields_named(ident, fields_named)
                }
                Fields::Unnamed(fields) => UnionVariant::from_syn_fields_unnamed(ident, fields),
                Fields::Unit => Some(UnionVariant::from_unit(ident)),
            }
        })
        .collect()
}

fn as_ty(ty: &Type) -> Result<Ty, String> {
    match ty {
        Type::Path(p) => {
            // is it a raw ident, like 'u8'
            if let Some(ident) = p.path.get_ident() {
                return Ok(rust_ident_to_ty(ident.to_string()));
            }

            for x in &p.path.segments {
                match &x.arguments {
                    PathArguments::None => {
                        todo!("PathArguments::None")
                    }
                    PathArguments::AngleBracketed(o) => {
                        let ident = x.ident.to_string();
                        let first_arg = o.args.first();

                        match (ident.as_str(), first_arg) {
                            ("Vec" | "Option" | "BTreeMap" | "HashMap", Some(arg1)) => {
                                if let Ok(inner) = ty_from_generic_argument(arg1) {
                                    if ident == "Vec" {
                                        return Ok(Ty::seq(inner));
                                    }
                                    if ident == "Option" {
                                        return Ok(Ty::optional(inner));
                                    }
                                    if (ident == "BTreeMap" || ident == "HashMap")
                                        && o.args.len() >= 2
                                    {
                                        let arg2 = o.args.index(1);
                                        if let Ok(inner2) = ty_from_generic_argument(arg2) {
                                            return Ok(Ty::record(inner, inner2));
                                        }
                                    }
                                }
                            }
                            _a => todo!("support more idents like: {}", ident),
                        }
                    }
                    PathArguments::Parenthesized(_) => {
                        println!("para")
                    }
                }
            }

            Err("could not get identifier".into())
        }
        Type::Tuple(t) if !t.elems.is_empty() => {
            let mut types = Vec::with_capacity(t.elems.len());
            for element in &t.elems {
                types.push(as_ty(element)?);
            }
            Ok(Ty::Tuple(types))
        }
        _ => Err(String::from("unknown")),
    }
}

fn ty_from_generic_argument(a: &GenericArgument) -> Result<Ty, String> {
    match a {
        GenericArgument::Type(ty) => as_ty(ty),
        _ => Err("only Types are supported as generic arguments".into()),
    }
}

fn quote<A: AsRef<str>>(a: A) -> String {
    format!("\"{}\"", a.as_ref())
}

fn serde_attrs(attrs: &[Attribute]) -> HashMap<String, String> {
    attrs
        .iter()
        .filter(|att| att.path.get_ident().filter(|v| *v == "serde").is_some())
        .filter_map(|item| {
            let parsed = item.parse_meta().expect("parse meta on attribute");
            if let Meta::List(l) = parsed {
                for nested in l.nested {
                    match nested {
                        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                            path,
                            lit: syn::Lit::Str(str),
                            ..
                        })) => {
                            if let Some(ident) = path.get_ident().map(|x| x.to_string()) {
                                return Some((ident, str.value()));
                            }
                        }
                        _ => todo!("?"),
                    }
                }
            }
            None
        })
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
    // println!("{}", raw_ident.as_ref());
    match raw_ident.as_ref() {
        "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64" | "isize" | "f32"
        | "f64" => Ty::ZodNumber,
        "bool" => Ty::ZodBoolean,
        "String" => Ty::ZodString,
        ident => Ty::Reference(ident.to_string()),
    }
}
