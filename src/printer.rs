extern crate termcolor;

use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

// TODO(simeg): Pass in stream to make it testable
pub struct Printer;

pub trait Print {
    fn println(&self, value: &str);
    fn print_input_header(&self, value: &str);
    fn print_editor_selection_header(&self);
    fn print_fts_banner(&self);
}

impl Print for Printer {
    fn println(&self, value: &str) {
        let mut stdout = StandardStream::stdout(ColorChoice::Never);
        writeln!(&mut stdout, "{}", value).expect("Could not write to stdout");
    }

    fn print_input_header(&self, value: &str) {
        println_w_opts(value, Color::Green, true);
        print("> ");
    }

    fn print_editor_selection_header(&self) {
        let text = "What editor do you want to use for writing down your ideas?";
        println_w_opts(text, Color::Green, true);
        print(""); // Don't make options for selecting editor also be colored
    }

    fn print_fts_banner(&self) {
        let color = Color::Yellow;
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
            println_w_opts(row, color, false);
        }
    }
}

fn print(value: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Never);
    write!(&mut stdout, "{}", value).expect("Could not write to stdout");
    std::io::stdout().flush().expect("Could not flush stdout");
}

fn println_w_opts(value: &str, color: Color, is_bold: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    let mut opts = ColorSpec::new();
    opts.set_fg(Some(color)).set_bold(is_bold);
    stdout
        .set_color(&opts)
        .expect("Could not set color for stdout");
    writeln!(&mut stdout, "{}", value).expect("Could not write to stdout");
    stdout.reset().expect("Could not reset stdout");
}
