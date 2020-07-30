#![allow(clippy::useless_let_if_seq)]
extern crate eureka;

#[macro_use]
extern crate clap;
extern crate termcolor;

use std::io;

use clap::{App, Arg, ArgMatches};
use termcolor::{ColorChoice, StandardStream};

use eureka::types::CliFlag::*;
use eureka::utils::exit_w_code;
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
            Arg::with_name(View.value())
                .long(View.value())
                .short(ShortView.value())
                .help("View ideas with your $PAGER env variable. If unset use less"),
        )
        .get_matches();

    let stdio = io::stdin();
    let input = stdio.lock();
    let output = StandardStream::stdout(ColorChoice::AlwaysAnsi);

    let mut eureka = Eureka::new(output, input);

    if cli_flags.is_present(ClearRepo.value()) {
        eureka.clear_repo();
        exit_w_code(0);
    }

    if cli_flags.is_present(View.value()) {
        eureka.open_idea_file();
        exit_w_code(0);
    }

    eureka.run()
}
