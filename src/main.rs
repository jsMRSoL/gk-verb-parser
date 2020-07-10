mod clap;
mod funcs;
mod types;
mod verbentry;
use crate::funcs::{check_file, parse_file};
use csv::Writer;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let clap_app = crate::clap::clap_app();

    // parse the launch arguments we got from cli
    let cli_matches = clap_app.get_matches();

    // if let Err(e) = parse_file() {
    //     eprintln!("Sorry! Pooched it.\n{}", e);
    // }
    let infile = cli_matches
        .value_of("infile")
        .expect("No input file given!");

    if !check_file(infile) {
        eprintln!("File {} contains accents. Please remove them.", infile);
        process::exit(1);
    }

    if let Ok(entries) = parse_file(infile) {
        if cli_matches.is_present("print") {
            for verb in entries {
                verb.pai.print();
                verb.ppi.print();
                verb.iai.print();
                verb.ipi.print();
                verb.fai.print();
                verb.fai2.print();
                verb.fmi.print();
                verb.fmi2.print();
                verb.fpi.print();
                verb.fpi2.print();
                verb.aai.print();
                verb.aai2.print();
                verb.ami.print();
                verb.ami2.print();
                verb.api.print();
                verb.api2.print();
            }
        } else if let Some(outfile) = cli_matches.value_of("outfile") {
            let mut wtr = Writer::from_path(outfile).expect("Could not create outfile.");
            for verb in entries {
                if let Some(conj) = verb.pai.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.ppi.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.iai.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.ipi.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.fai.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.fai2.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.fmi.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.fmi2.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.fpi.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.fpi2.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.aai.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.aai2.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.ami.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.ami2.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.api.to_vec() {
                    wtr.write_record(conj)?;
                }
                if let Some(conj) = verb.api2.to_vec() {
                    wtr.write_record(conj)?;
                }
            }
            wtr.flush()?;
        }
    }
    Ok(())
}
