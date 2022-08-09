use crate::{Context, Print};
use std::fmt::Write;

#[derive(Debug)]
pub struct Import {
    pub ident: String,
    pub path: String,
}

impl Print for Import {
    fn print(&self, x: &mut String, _ctx: &Context) -> Result<(), std::fmt::Error> {
        writeln!(
            x,
            "import {} from {};",
            self.ident,
            crate::quote(&self.path)
        )
    }
}

impl Print for Vec<Import> {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error> {
        for import in self {
            import.print(x, ctx)?;
        }
        Ok(())
    }
}
