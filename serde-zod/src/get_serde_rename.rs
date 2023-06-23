use quote::ToTokens;
use syn::Attribute;

pub fn get_serde_rename(attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs {
        if (attr.path.segments.len() != 1) || (attr.path.segments.first().unwrap().ident != "serde")
        {
            continue;
        }
        let Ok(input) = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()) else {
            continue;
        };
        let mut stream = input.stream().into_iter();
        let Some(indent) = stream.next() else { continue };
        let Ok(indent) = syn::parse2::<proc_macro2::Ident>(indent.into_token_stream()) else { continue };
        if &indent.to_string() != "rename" {
            continue;
        }
        let Some(punct) = stream.next() else { continue };
        let Ok(punct) = syn::parse2::<proc_macro2::Punct>(punct.into_token_stream()) else { continue };
        if &punct.to_string() != "=" {
            continue;
        }
        let Some(literal) = stream.next() else { continue };
        let Ok(literal) = syn::parse2::<proc_macro2::Literal>(literal.into_token_stream()) else { continue };

        let new_name = literal.to_string();
        let Some(new_name) = new_name.strip_suffix('"') else { continue };
        let Some(new_name) = new_name.strip_prefix('"') else { continue };
        return Some(new_name.to_string());
    }
    None
}
