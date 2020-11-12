#[macro_use]
extern crate clap;
extern crate termcolor;

use std::{io, process};

use clap::{App, Arg, ArgMatches};
use termcolor::{ColorChoice, StandardStream};

use eureka::file_handler::FileHandler;
use eureka::printer::Printer;
use eureka::program_access::ProgramAccess;
use eureka::reader::Reader;
use eureka::types::CliFlag::*;
use eureka::{Eureka, EurekaOptions};

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
            Arg::with_name(ClearBranch.value())
                .long(ClearBranch.value())
                .help("Clear the stored branch name"),
        )
        .arg(
            Arg::with_name(View.value())
                .long(View.value())
                .short(ShortView.value())
                .help("View ideas with your $PAGER env variable. If unset use less"),
        )
        .get_matches();

    let stdio = io::stdin();
    let input = stdio.lock();
    let output = StandardStream::stdout(ColorChoice::AlwaysAnsi);

    let mut eureka = Eureka::new(
        FileHandler::default(),
        Printer::new(output),
        Reader::new(input),
        ProgramAccess::default(),
    );

    let opts = EurekaOptions {
        clear_repo: cli_flags.is_present(ClearRepo.value()),
        clear_branch: cli_flags.is_present(ClearBranch.value()),
        view: cli_flags.is_present(View.value()),
    };

    let status_code = match eureka.run(opts) {
        Ok(_) => 0,
        Err(_) => 1,
    };

    process::exit(status_code)
}
