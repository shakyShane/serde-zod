extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenTree};
use std::collections::HashMap;

use quote::__private::ext::RepToTokensExt;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{parsing, Comma};
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Error, Field, Fields, Lit, Meta,
    MetaList, NestedMeta,
};

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    let tokens = quote! {
        struct Hello;
    };

    tokens.into()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
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

    if serde_attr.len() > 0 {
        println!("has serde attrs {}", serde_attr.len());
        match &input_parsed.data {
            Data::Struct(_) => {}
            Data::Enum(e) => process_tagged_enum(&input_parsed.ident, e, "kind"),
            Data::Union(_) => {}
        }
    }

    let tokens = quote! {
        #input_parsed
        struct Hello;
    };

    tokens.into()
}

fn process_tagged_enum(ident: &Ident, e: &DataEnum, tag: &str) {
    let mut tu = zod::TaggedUnion::default();
    tu.ident = ident.to_string();
    tu.tag = tag.to_string();

    e.variants.iter().for_each(|vari| {
        // println!("variant ident: {}", vari.ident);
        match &vari.fields {
            Fields::Named(named) => unreachable!("named not yet supported"),
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

    tu.print();

    // dbg!(tu);
}

mod zod {
    #[derive(Default, Debug)]
    pub struct TaggedUnionVariant {
        pub ident: String,
        pub fields: TaggedUnionFields,
    }
    #[derive(Default, Debug)]
    pub enum TaggedUnionFields {
        #[default]
        Unit,
    }

    #[derive(Default, Debug)]
    pub struct TaggedUnion {
        pub ident: String,
        pub tag: String,
        pub variants: Vec<TaggedUnionVariant>,
    }

    impl TaggedUnion {
        pub fn print(&self) {
            let mut lines = vec![];
            lines.push(format!("export const {} = z", self.ident));
            lines.push(format!(r#"  .discriminatedUnion("{}", ["#, self.tag));
            for x in &self.variants {
                lines.push(format!("    z.object({{"));
                lines.push(format!(
                    "      {}: {}",
                    self.tag,
                    format!(r#""{}""#, x.ident)
                ));
                // todo: "push other fields"
                lines.push(format!("    }}),"));
            }
            lines.push(format!(r#"  ])"#));

            println!("{}", lines.join("\n"));
        }
    }
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

struct SerdeAttr {
    tag: String,
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
