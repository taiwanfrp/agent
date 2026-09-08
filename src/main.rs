mod i18n;
mod language;

use clap::Parser;

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
}
