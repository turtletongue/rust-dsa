use std::path:: PathBuf;

use clap::{arg, command, value_parser, Command};

use rust_dsa::operations::{run_keys_generation, run_signing, run_verifying};

fn main() {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("keygen")
                .about("Generate public and private keys")
                .arg(
                    arg!(-p --path <PATH> "Sets path to directory where keys should be created")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("sign")
                .about("Sign a message")
                .arg(
                    arg!(--private <PATH> "Sets path to private key file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    arg!(--public <PATH> "Sets path to public key file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    arg!(-m --message <TEXT> "Sets message text")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("verify")
                .about("Verify message signature")
                .arg(
                    arg!(--public <PATH> "Sets path to public key file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    arg!(-m --message <TEXT> "Sets message text")
                        .required(true)
                )
                .arg(
                    arg!(--r <R> "Sets r parameter")
                        .required(false)
                )
                .arg(
                    arg!(--s <S> "Sets s parameter")
                        .required(false)
                )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("keygen") {
        return run_keys_generation(matches.get_one::<PathBuf>("path").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("sign") {
        return run_signing(
            matches.get_one::<PathBuf>("public").unwrap(),
            matches.get_one::<PathBuf>("private").unwrap(),
            matches.get_one::<String>("message").unwrap().to_string()
        );
    }

    if let Some(matches) = matches.subcommand_matches("verify") {
        return run_verifying(
            matches.get_one::<PathBuf>("public").unwrap(),
            matches.get_one::<String>("message").unwrap().to_string(),
            matches.get_one::<String>("r").unwrap().to_string(),
            matches.get_one::<String>("s").unwrap().to_string()
        )
    }

}