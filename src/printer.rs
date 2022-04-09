use std::io;
use std::io::Write;

pub trait Print {
    fn print(&mut self, value: &str) -> io::Result<()>;
    fn println(&mut self, value: &str) -> io::Result<()>;
}

pub trait PrintColor {
    fn fts_banner(&mut self) -> io::Result<()>;
    fn input_header(&mut self, value: &str) -> io::Result<()>;
    fn error(&mut self, value: &str) -> io::Result<()>;
}

pub struct Printer<W> {
    writer: W,
}

#[derive(Clone, Copy)]
pub struct PrintOptions {
    color: termcolor::Color,
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
            color: termcolor::Color::Yellow,
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
            color: termcolor::Color::Green,
            is_bold: true,
        };
        self.println_styled(value, opts)?;
        self.print("> ")?;
        self.writer.flush()
    }

    fn error(&mut self, value: &str) -> io::Result<()> {
        let opts = PrintOptions {
            color: termcolor::Color::Red,
            is_bold: false,
        };
        self.println_styled(value, opts)?;
        self.writer.flush()
    }
}

impl<W: Write + termcolor::WriteColor> Printer<W> {
    fn println_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()> {
        let mut color_spec = termcolor::ColorSpec::new();
        color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
        self.writer.set_color(&color_spec)?;
        writeln!(self.writer, "{}", value)?;
        self.writer.reset()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::printer::{Print, PrintColor, PrintOptions, Printer};

    #[test]
    fn test_printer__print__success() {
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
    fn test_printer__println__success() {
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

    #[test]
    fn test_printer__fts_banner__success() {
        let mut output = termcolor::Ansi::new(vec![]);
        let mut printer = Printer::new(&mut output);

        printer.fts_banner().unwrap();

        let actual = String::from_utf8(output.into_inner()).unwrap();
        let expected = "############################################################
####                  First Time Setup                  ####
############################################################

This tool requires you to have a repository with a README.md
in the root folder. The markdown file is where your ideas
will be stored.

Once first time setup has completed, simply run Eureka again
to begin writing down ideas.";

        assert!(actual.starts_with("\u{1b}[0m\u{1b}[33m"));
        assert!(actual.contains(expected));
        assert!(actual.ends_with("\n\u{1b}[0m"));
    }

    #[test]
    fn test_printer__input_header__success() {
        let mut output = termcolor::Ansi::new(vec![]);
        let mut printer = Printer::new(&mut output);

        printer.input_header("some-value").unwrap();

        let actual = String::from_utf8(output.into_inner()).unwrap();
        let expected = "\u{1b}[0m\u{1b}[1m\u{1b}[32msome-value\n\u{1b}[0m> ";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_printer__error__success() {
        let mut output = termcolor::Ansi::new(vec![]);
        let mut printer = Printer::new(&mut output);

        printer.error("some-value").unwrap();

        let actual = String::from_utf8(output.into_inner()).unwrap();
        let expected = "\u{1b}[0m\u{1b}[31msome-value\n\u{1b}[0m";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_printer__println_styled__success() {
        let mut output_1 = termcolor::Ansi::new(vec![]);
        let mut printer = Printer::new(&mut output_1);

        let opts_green_bold = PrintOptions {
            color: termcolor::Color::Green,
            is_bold: true,
        };

        printer
            .println_styled("some-green-bold-text", opts_green_bold)
            .unwrap();

        let actual_green_bold = String::from_utf8(output_1.into_inner()).unwrap();
        let expected_green_bold = "\u{1b}[0m\u{1b}[1m\u{1b}[32msome-green-bold-text\n\u{1b}[0m";

        assert_eq!(actual_green_bold, expected_green_bold);

        let mut output_2 = termcolor::Ansi::new(vec![]);
        printer = Printer::new(&mut output_2);

        let opts_yellow = PrintOptions {
            color: termcolor::Color::Yellow,
            is_bold: false,
        };

        printer
            .println_styled("some-yellow-text", opts_yellow)
            .unwrap();

        let actual_yellow = String::from_utf8(output_2.into_inner()).unwrap();
        let expected_yellow = "\u{1b}[0m\u{1b}[33msome-yellow-text\n\u{1b}[0m";

        assert_eq!(actual_yellow, expected_yellow);
    }
}
