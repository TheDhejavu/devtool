use log::*;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

use clap::{Arg, ArgMatches, Command};
use lang::Language;

mod builder;
mod lang;
mod os;
mod tools;

fn create_cmd(sub_matches: &ArgMatches) {
    let project_name = sub_matches
        .get_one::<String>("project")
        .expect("required")
        .as_str();

    let lang = sub_matches
        .get_one::<String>("language")
        .expect("required")
        .as_str();

    info!("Create Project: {} Language: {}", project_name, lang);
    match lang {
        Language::NODE => println!("NODE"),
        Language::PYTHON => tools::install_deps(Language::PYTHON, project_name),
        Language::JAVA => println!("JAVA"),
        Language::GO => tools::install_deps(Language::GO, project_name),
        _ => error!("language is currently not supported"),
    }
}

fn update_cmd(_sub_matches: &ArgMatches) {
    info!("Deploying...");
}

fn deploy_cmd(sub_matches: &ArgMatches) {
    let project_name = sub_matches
        .get_one::<String>("project")
        .expect("required")
        .as_str();

    let lang = sub_matches
        .get_one::<String>("language")
        .expect("required")
        .as_str();

    info!("Update Project: {} Language: {}", project_name, lang);
}

fn cli() -> Command {
    Command::new("devtool")
        .about("A basic CLI tool for installing developer tools for different environment and programming language (Mac, Linux, Windows). ")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("create")
                .about("Initialized the specified language dependencies and boilerplate code")
                .arg(
                    Arg::new("language")
                        .short('l')
                        .long("lang")
                )
                .arg(
                    Arg::new("project")
                        .short('p')
                        .long("project")
                )
        )
        .subcommand(
            Command::new("update")
                .about("Update project dependencies")
                .arg(
                    Arg::new("language")
                        .short('l')
                        .long("lang")
                )
                .arg(
                    Arg::new("project")
                        .short('p')
                        .long("project")
                )
        )

        .subcommand(
            Command::new("deploy")
                .about("Deploy project")
        )
}

fn main() {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => create_cmd(sub_matches),
        Some(("update", sub_matches)) => update_cmd(sub_matches),
        Some(("deploy", sub_matches)) => deploy_cmd(sub_matches),
        _ => unreachable!(),
    }
}
