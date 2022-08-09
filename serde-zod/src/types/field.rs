use crate::{as_ty, Context, Print, Ty};
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Field {
    pub ident: String,
    pub ty: Ty,
}

impl From<(String, Ty)> for Field {
    fn from((ident, ty): (String, Ty)) -> Self {
        Self { ident, ty }
    }
}

impl From<(&String, &Ty)> for Field {
    fn from((ident, ty): (&String, &Ty)) -> Self {
        Self {
            ident: ident.into(),
            ty: ty.clone(),
        }
    }
}

impl Field {
    pub fn new(ident: impl Into<String>, ty: Ty) -> Self {
        Self {
            ident: ident.into(),
            ty,
        }
    }
    pub fn from_syn_field(field: &syn::Field) -> Option<Self> {
        match (&field.ident, as_ty(&field.ty).ok()) {
            (Some(ident), Some(ty)) => Some(Self::new(ident.to_string(), ty)),
            _ => None,
        }
    }
}

impl Print for Field {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        let ty_string = self.ty.as_string(ctx)?;
        write!(x, "{}: {}", self.ident, ty_string)
    }
}
