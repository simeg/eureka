#[macro_use]
extern crate clap;
extern crate pretty_env_logger;
extern crate termcolor;

use std::io;

use eureka::config_manager::ConfigManager;
use eureka::git::Git;
use eureka::printer::Printer;
use eureka::program_access::ProgramAccess;
use eureka::reader::Reader;
use eureka::{Eureka, EurekaOptions};
use log::error;

const ARG_CLEAR_CONFIG: &str = "clear-config";
const ARG_VIEW: &str = "view";

fn main() {
    pretty_env_logger::init();

    let cli_flags = clap::Command::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        .arg(
            clap::Arg::new(ARG_CLEAR_CONFIG)
                .long(ARG_CLEAR_CONFIG)
                .help("Clear your stored configuration"),
        )
        .arg(
            clap::Arg::new(ARG_VIEW)
                .long(ARG_VIEW)
                .short(ARG_VIEW.chars().next().unwrap())
                .help("View ideas with your $PAGER env variable. If unset use less"),
        )
        .get_matches();

    let stdio = io::stdin();
    let input = stdio.lock();
    let output = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);

    let mut eureka = Eureka::new(
        ConfigManager::default(),
        Printer::new(output),
        Reader::new(input),
        Git::default(),
        ProgramAccess::default(),
    );

    let opts = EurekaOptions {
        clear_config: cli_flags.is_present(ARG_CLEAR_CONFIG),
        view: cli_flags.is_present(ARG_VIEW),
    };

    match eureka.run(opts) {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    }
}
