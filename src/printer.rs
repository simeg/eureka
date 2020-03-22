extern crate termcolor;

use std::io;

use self::termcolor::{Color, ColorSpec};

pub struct Printer<W> {
    pub writer: W,
}

pub trait Print {
    fn println(&mut self, value: &str);
    fn print_input_header(&mut self, value: &str);
    fn print_editor_selection_header(&mut self);
    fn print_fts_banner(&mut self);
    fn flush(&mut self) -> io::Result<()>;
}

impl<W: io::Write + termcolor::WriteColor> Print for Printer<W> {
    fn println(&mut self, value: &str) {
        writeln!(self.writer, "{}", value).expect("Could not write to stdout");
    }

    fn print_input_header(&mut self, value: &str) {
        let opts = PrintOptions {
            color: Color::Green,
            is_bold: true,
        };
        println(&mut self.writer, value, opts);
        print(&mut self.writer, "> ");
    }

    fn print_editor_selection_header(&mut self) {
        let opts = PrintOptions {
            color: Color::Green,
            is_bold: true,
        };
        let text = "What editor do you want to use for writing down your ideas?";
        println(&mut self.writer, text, opts);
        print(&mut self.writer, "");
    }

    fn print_fts_banner(&mut self) {
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
            println(&mut self.writer, row, opts);
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

fn print<W>(stdout: &mut W, value: &str)
where
    W: io::Write,
{
    write!(stdout, "{}", value).expect("Could not write to stdout");
    stdout.flush().expect("Could not flush stdout");
}

#[derive(Clone, Copy)]
struct PrintOptions {
    color: Color,
    is_bold: bool,
}

fn println<W>(stdout: &mut W, value: &str, opts: PrintOptions)
where
    W: io::Write + termcolor::WriteColor,
{
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
    stdout
        .set_color(&color_spec)
        .expect("Could not set color for stdout");
    writeln!(stdout, "{}", value).expect("Could not write to stdout");
    stdout.reset().expect("Could not reset stdout");
}
