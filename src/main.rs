use clap::{App, Arg, SubCommand, crate_version, AppSettings};

use {{crate_name}}::*;
use std::io;
use std::io::stdin;

fn main() -> io::Result<()> {
    let matches = App::new("{{project-name}}")
        .version(crate_version!())
        .author("{{authors}}")
        .about("TODO: a little description would be nice")
        .setting(AppSettings::ArgRequiredElseHelp)
    ).subcommand(SubCommand::with_name("TODO a basic subcommand")
        .about("TODO briefly what it does")
        .arg(
            Arg::with_name("argname1")
                .short("i")
                .long("in")
                .value_name("name of argname1")
                .takes_value(true)
                .required(true)
                .help("Some more expressive help text"),
        )
    ).get_matches();

    match matches.subcommand() {
        ("TODO a basic subcommand", Some(m)) => {
            if let Some(arg) = m.value_of("argname1") {
                // TODO do something with this argument
            }

            // TODO final implementation
        }
        _ => { }
    }

    Ok(())
}
