use crate::termcolor::{Color, ColorSpec};

use std::io::Write;

pub trait Print {
    fn print(&mut self, value: &str);
    fn println(&mut self, value: &str);
}

pub struct Printer<W> {
    writer: W,
}

#[derive(Clone, Copy)]
pub struct PrintOptions {
    color: Color,
    is_bold: bool,
}

impl<W: Write + termcolor::WriteColor> Printer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn input_header(&mut self, value: &str) {
        let opts = PrintOptions {
            color: Color::Green,
            is_bold: true,
        };
        self.println_styled(value, opts);
        self.print("> ");
        self.writer.flush().expect("Could not flush");
    }

    pub fn fts_banner(&mut self) {
        let opts = PrintOptions {
            color: Color::Yellow,
            is_bold: false,
        };
        let banner = format!(
            "{}\n{}{}{}{}{}\n{}",
            "#".repeat(60),
            "#".repeat(4),
            " ".repeat(18),
            "First Time Setup",
            " ".repeat(18),
            "#".repeat(4),
            "#".repeat(60)
        );
        let row0 = "";
        let row1 = "This tool requires you to have a repository with a README.md";
        let row2 = "in the root folder. The markdown file is where your ideas";
        let row3 = "will be stored. ";
        let row4 = "";
        let row5 = "Once first time setup has completed, simply run Eureka again";
        let row6 = "to begin writing down ideas.";
        let row7 = "";
        let rows = [
            banner.as_str(),
            row0,
            row1,
            row2,
            row3,
            row4,
            row5,
            row6,
            row7,
        ];
        for row in &rows {
            self.println_styled(row, opts);
        }
    }

    fn println_styled(&mut self, value: &str, opts: PrintOptions) {
        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
        self.writer
            .set_color(&color_spec)
            .expect("Could not set color for stdout");
        writeln!(self.writer, "{}", value).expect("Could not write to stdout");
        self.writer.reset().expect("Could not reset stdout");
    }
}

impl<W: Write> Print for Printer<W> {
    fn print(&mut self, value: &str) {
        write!(self.writer, "{}", value).expect("Could not write to stdout");
    }

    fn println(&mut self, value: &str) {
        writeln!(self.writer, "{}", value).expect("Could not write to stdout");
    }
}
