#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate eureka;

#[macro_use]
extern crate clap;
extern crate dialoguer;
extern crate termcolor;

use std::io;

use clap::{App, Arg, ArgMatches};
use termcolor::{ColorChoice, StandardStream};

use eureka::file_handler::FileHandler;
use eureka::printer::Printer;
use eureka::reader::Reader;
use eureka::types::CliFlag::*;
use eureka::utils::utils::exit_w_code;
use eureka::Eureka;

fn main() {
    let cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        .arg(
            Arg::with_name(ClearRepo.value())
                .long(ClearRepo.value())
                .help("Clear the stored path to your idea repo"),
        )
        .arg(
            Arg::with_name(ClearEditor.value())
                .long(ClearEditor.value())
                .help("Clear the stored path to your idea editor"),
        )
        .arg(
            Arg::with_name(View.value())
                .long(View.value())
                .short(ShortView.value())
                .help("View your ideas using less"),
        )
        .get_matches();

    let stdio = io::stdin();
    let input = stdio.lock();
    let output = StandardStream::stdout(ColorChoice::AlwaysAnsi);

    let mut eureka = Eureka {
        fh: FileHandler {},
        // TODO: Replace reader and writer with single Environment/IO type?
        reader: Reader { reader: input },
        printer: Printer { writer: output },
    };

    if eureka.handle_flags(cli_flags).is_err() {
        exit_w_code(0);
    }

    eureka.run();
}
