use clap::{arg, Command};

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

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            println!(
                "Initalizing.... {}",
                sub_matches.get_one::<String>("LANG").expect("required")
            );
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
