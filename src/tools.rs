use std::env;
use std::process::{Command, Stdio};
struct OS;

impl OS {
    pub const WINDOW: &str = "window";
    pub const LINUX: &str = "linux";
    pub const MACOS: &str = "macos";
}

pub fn install_python_deps() {
    let os = env::consts::OS;
    println!("Installing python deps...... {}", os);

    match os {
        OS::MACOS => {
            if !is_docker_installed() {
                // Install brew
                if !is_brew_installed() {
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

                println!("Installing docker...");
                let mut out = Command::new("brew")
                    .arg("install")
                    .arg("--cask")
                    .arg("docker")
                    .spawn()
                    .expect("failed to install docker");
                out.wait().expect("Failing while waiting");

                println!("{:?}", out);
            }
        }
        OS::LINUX => println!("<MACOS>"),
        OS::WINDOW => println!("WINDOW"),
        _ => println!("OS is currently not supported"),
    }
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
