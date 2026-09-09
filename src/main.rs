mod credentials;
mod i18n;
mod language;

use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};

#[derive(Parser)]
#[command(version)]
#[command(before_help = "TaiwanFRP Agent", about)]
struct Cli {
    /// Show the system language
    #[arg(long)]
    lang: bool,
}

fn main() {
    let cli = Cli::parse();

    let language = language::get_system_locale();

    if cli.lang {
        println!("{}", language);
        return;
    }

    let i18n = i18n::load_language(&language);

    println!("{}", i18n.get("welcome"));

    match credentials::load_credentials() {
        Ok((username, password)) => {
            println!("{}", i18n.get("auth.credentials_found"));
            println!("{}{}", i18n.get("auth.username"), username);
            println!("{}{}", i18n.get("auth.password"), password);
        }

        Err(_) => {
            println!("{}", i18n.get("auth.credentials_not_found").yellow());

            print!("{}", i18n.get("auth.username"));
            io::stdout().flush().unwrap();

            let mut username = String::new();
            io::stdin().read_line(&mut username).unwrap();
            let username = username.trim();

            print!("{}", i18n.get("auth.password"));
            io::stdout().flush().unwrap();

            let password = rpassword::read_password().unwrap();

            credentials::save_credentials(username, &password).expect("Failed to save credentials");
            println!("{}", i18n.get("auth.credentials_saved"));
        }
    }
}
