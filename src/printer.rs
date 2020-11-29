use crate::termcolor::{Color, ColorSpec};

use std::io;
use std::io::Write;

pub trait Print {
    fn print(&mut self, value: &str) -> io::Result<()>;
    fn println(&mut self, value: &str) -> io::Result<()>;
}

pub trait PrintColor {
    fn fts_banner(&mut self) -> io::Result<()>;
    fn input_header(&mut self, value: &str) -> io::Result<()>;
    fn println_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()>;
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
}

impl<W: Write> Print for Printer<W> {
    fn print(&mut self, value: &str) -> io::Result<()> {
        write!(self.writer, "{}", value)
    }

    fn println(&mut self, value: &str) -> io::Result<()> {
        writeln!(self.writer, "{}", value)
    }
}

impl<W: Write + termcolor::WriteColor> PrintColor for Printer<W> {
    fn fts_banner(&mut self) -> io::Result<()> {
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
        let description = r#"
This tool requires you to have a repository with a README.md
in the root folder. The markdown file is where your ideas
will be stored.

Once first time setup has completed, simply run Eureka again
to begin writing down ideas.
        "#;
        self.println_styled(&format!("{}\n{}", banner.as_str(), description), opts)
    }

    fn input_header(&mut self, value: &str) -> io::Result<()> {
        let opts = PrintOptions {
            color: Color::Green,
            is_bold: true,
        };
        self.println_styled(value, opts)?;
        self.print("> ")?;
        self.writer.flush()
    }

    fn println_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()> {
        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
        self.writer.set_color(&color_spec)?;
        writeln!(self.writer, "{}", value)?;
        self.writer.reset()
    }
}

#[cfg(test)]
mod tests {
    use crate::printer::{Print, Printer};

    #[test]
    fn test_print_works() {
        let mut output = Vec::new();
        let mut printer = Printer {
            writer: &mut output,
        };

        let print_result = printer.print("this value");
        assert!(print_result.is_ok());

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "this value";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_println_works() {
        let mut output = Vec::new();
        let mut printer = Printer {
            writer: &mut output,
        };

        let print_result = printer.println("this value");
        assert!(print_result.is_ok());

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "this value\n";

        assert_eq!(actual, expected);
    }
}
