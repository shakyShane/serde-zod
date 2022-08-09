use crate::indent::indent_all_by;
use std::fmt::Write;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum OutputType {
    #[default]
    Serialize,
    Deserialize,
}

#[derive(Debug, Default)]
pub struct Context {
    pub output_type: OutputType,
}

pub trait Print {
    fn print(&self, x: &mut String, ctx: &Context) -> Result<(), std::fmt::Error>;
    fn as_string(&self, ctx: &Context) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.print(&mut s, ctx)?;
        Ok(s)
    }
}

pub struct Printer {
    lines: Vec<String>,
    buffer: String,
    curr: usize,
    size: usize,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            buffer: String::new(),
            curr: 0,
            size: 2,
        }
    }
    pub fn indent(&mut self) {
        self.curr += self.size
    }
    pub fn dedent(&mut self) {
        if self.curr >= self.size {
            self.curr -= self.size
        }
    }
    pub fn writeln<A: AsRef<str>>(&mut self, p: A) -> Result<(), std::fmt::Error> {
        writeln!(self.buffer, "{}", indent_all_by(self.curr, p.as_ref()))
    }
    pub fn write<A: AsRef<str>>(&mut self, p: A) -> Result<(), std::fmt::Error> {
        write!(self.buffer, "{}", indent_all_by(self.curr, p.as_ref()))
    }
    pub fn line(&mut self, line: impl Into<String>) {
        self.lines.push(indent_all_by(self.curr, line.into()));
    }
    pub fn join_lines(&mut self, join_char: char) -> Result<(), std::fmt::Error> {
        if self.lines.is_empty() {
            return Ok(());
        }
        let indented = self
            .lines
            .iter()
            .map(|l| {
                let last = &l[l.len() - 1..l.len()];
                if last == "\n" {
                    let without = &l[0..l.len() - 1];
                    format!("{}{}", without, join_char)
                } else {
                    format!("{}{}", l, join_char)
                }
                // if let Some('\n') = l.chars().last() {
                // } else {
                // }
            })
            .collect::<Vec<_>>()
            .join("\n");
        writeln!(self.buffer, "{}", indented)?;
        self.lines.drain(..);
        Ok(())
    }
    // move ownership
    pub fn dump(self) -> String {
        self.buffer
    }
}

#[test]
fn test_printer() -> Result<(), std::fmt::Error> {
    let mut printer = Printer::new();
    printer.writeln("export const obj = z")?;
    printer.indent();
    printer.writeln(".object({")?;
    printer.indent();
    printer.line("age: z.number()");
    printer.join_lines(',')?;
    printer.join_lines(',')?;
    printer.dedent();
    printer.writeln("})")?;
    let output = printer.dump();
    println!("|{}|", output);
    Ok(())
}

pub trait Container {
    fn display_ident(&self) -> &String;
}
