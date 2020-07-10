use clap::{App, Arg};

pub fn clap_app() -> clap::App<'static, 'static> {
    App::new("Greek Verb Conjugator")
        .arg(
            Arg::with_name("infile")
                .help("File to read from")
                .index(1)
                .required(true)
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outfile")
                .help("File to write to")
                .short("o")
                .long("outfile")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("print")
                .help("Print to stdout")
                .short("p")
                .long("print")
                .takes_value(false),
        )
}
