use log::{error, info};
use std::env;
use std::process::{Command, Stdio};

use crate::builder;
use crate::lang::Language;
use crate::os::*;

pub fn install_deps(lang: &str, project_name: &str) {
    let os = env::consts::OS;
    info!("Installing {} deps for {}", os, lang);

    match os {
        OS::MACOS => {
            install_mac_os_deps();
            match lang {
                Language::GO => builder::build_go_boilerplate(project_name),
                _ => info!("language is currently not supported"),
            }
        }
        OS::LINUX => println!("</LINUX>"),
        OS::WINDOW => println!("</WINDOW>"),
        _ => error!("OS is currently not supported"),
    }
}

pub fn install_mac_os_deps() {
    if !is_docker_installed() {
        // Install brew
        if !is_brew_installed() {
            install_brew()
        }

        // install docker
        install_docker()
    }
}

pub fn install_brew() {
    let mut out = Command::new("/bin/bash")
        .arg("-c")
        .arg("https://raw.githubusercontent.com/Homebrew/install/master/install.sh")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute process");
    out.wait().expect("Failing while waiting");

    let mut out = Command::new("/bin/bash")
        .arg("./install.sh")
        .spawn()
        .expect("failed to execute process");
    out.wait().expect("Failing while waiting");

    let mut out = Command::new("rm")
        .arg("-rf")
        .arg("./install.sh")
        .spawn()
        .expect("failed to execute process");
    out.wait().expect("Failing while waiting");
}

pub fn install_docker() {
    info!("Installing docker...");
    let mut out = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("docker")
        .spawn()
        .expect("failed to install docker");
    out.wait().expect("Failing while waiting");

    info!("{:?}", out);
}

pub fn is_docker_installed() -> bool {
    let output = Command::new("docker")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    return output.is_ok();
}

pub fn is_brew_installed() -> bool {
    let output = Command::new("brew")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    return output.is_ok();
}
