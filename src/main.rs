use clap::{arg, Command};
mod builder;
mod tools;
mod util;

#[non_exhaustive]
struct Language;

impl Language {
    pub const PYTHON: &str = "python";
    pub const NODE: &str = "node";
    pub const GO: &str = "go";
    pub const JAVA: &str = "java";
}

fn cli() -> Command {
    Command::new("devtool")
        .about("A basic CLI tool for installing developer tools for different environment and programming language (Mac, Linux, Windows). ")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialized the specified language dependencies")
                .arg(arg!(<LANG> "Programming Language"))
                .arg_required_else_help(true),
        )
}
fn handle_init(value: &str) {
    match value {
        Language::NODE => println!("NODE"),
        Language::PYTHON => tools::install_deps(Language::PYTHON),
        Language::JAVA => println!("JAVA"),
        Language::GO => tools::install_deps(Language::GO),
        _ => println!("language is currently not supported"),
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let value = sub_matches
                .get_one::<String>("LANG")
                .expect("required")
                .as_str();

            handle_init(value)
        }
        Some(("update", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.get_one::<String>("LANG").expect("required")
            );
        }
        Some(("deps", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.get_one::<String>("LANG").expect("required")
            );
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}
