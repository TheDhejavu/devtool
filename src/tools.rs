use log::{error, info};
use std::env;
use std::process::{Command, Output};

use crate::builder;
use crate::lang::Language;
use crate::os::*;

/// Installs operating system dependencies.
pub fn install_deps(lang: &str, project_name: &str) {
    let os = env::consts::OS;
    info!("Installing {} deps for {}", os, lang);

    match os {
        OS::LINUX => install_docker_on_linux(),
        OS::MACOS => install_docker_on_macos(),
        OS::WINDOW => install_docker_on_windows(),
        _ => {
            error!("Unsupported operating system: {}", os);
        }
    }

    match lang {
        Language::GO => builder::build_go_boilerplate(project_name),
        _ => error!("language is currently not supported"),
    }
}

fn install_docker_on_linux() {
    let check_output = run_command("docker", &["--version"]);

    if check_output.is_ok() && check_output.unwrap().status.success() {
        info!("Docker is already installed on Linux!");
    } else {
        let output = run_command(
            "curl",
            &["-fsSL", "https://get.docker.com/", "-o", "get-docker.sh"],
        );

        if output.is_err() || !output.unwrap().status.success() {
            error!("Failed to download Docker installation script");
            return;
        }

        let install_output = run_command("sh", &["get-docker.sh"]);

        if install_output.is_ok() && install_output.unwrap().status.success() {
            info!("Docker installed successfully on Linux!");
        } else {
            error!("Failed to install Docker on Linux");
        }
    }
}

fn install_docker_on_macos() {
    let check_output = run_command("docker", &["--version"]);

    if check_output.is_ok() && check_output.unwrap().status.success() {
        info!("Docker is already installed on macOS!");
    } else {
        if let Ok(check_brew_output) = run_command("brew", &["--version"]) {
            if check_brew_output.status.success() {
                let output = run_command("brew", &["install", "docker"]);

                if output.is_ok() && output.unwrap().status.success() {
                    info!("Docker installed successfully on macOS!");
                    return;
                }
            }
        }

        let output = run_command(
            "curl",
            &[
                "-fsSL",
                "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
            ],
        );

        if output.is_err()  {
            error!("Failed to download Homebrew installation script");
            return;
        }

        let out = output.unwrap();
        if !out.status.success() {
            error!("Failed to download Homebrew installation script");
            return;
        }

        let install_output = run_command(
            "sh",
            &[&String::from_utf8_lossy(&out.stdout).to_string()],
        );

        if install_output.is_ok() && install_output.unwrap().status.success() {
            let docker_output = run_command("brew", &["install", "docker"]);

            if docker_output.is_ok() && docker_output.unwrap().status.success() {
                info!("Docker installed successfully on macOS!");
            } else {
                error!("Failed to install Docker on macOS");
            }
        } else {
            error!("Failed to install Homebrew");
        }
    }
}

fn install_docker_on_windows() {
    let check_output = run_command("docker", &["--version"]);

    if check_output.is_ok() && check_output.unwrap().status.success() {
        println!("Docker is already installed on Windows!");
    } else {
        let output = run_command("choco", &["install", "docker-cli"]);

        if output.is_ok() && output.unwrap().status.success() {
            println!("Docker installed successfully on Windows!");
        } else {
            println!("Failed to install Docker on Windows");
        }
    }
}

fn run_command(command: &str, args: &[&str]) -> Result<Output, std::io::Error> {
    Command::new(command).args(args).output()
}

pub fn generate_docker_compose(project_name: &str) -> String {
    // Define the Docker Compose file contents
    let docker_compose_contents = format!(
        "version: '3.8'
services:
  {project_name}:
    build:
      context: .
    ports:
      - '8080:8080'
    depends_on:
      - db
      - redis
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: {project_name}_db
      DATABASE_URL: postgres://postgres:postgres@db:5432/{project_name}_db
      REDIS_URL: redis://redis:6379/0
  db:
    image: postgres:latest
    restart: always
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: {project_name}_db
  redis:
    image: redis:latest
    restart: always
    ports:
      - '6379:6379'",
        project_name = project_name,
    );

    return docker_compose_contents;
}

pub fn generate_docker_file(project_name: &str) -> String {
    // Define the Dockerfile contents
    let dockerfile_contents = format!(
        "FROM golang:1.17-alpine3.14 AS builder
WORKDIR /go/src/{project_name}
COPY . .
RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o {project_name} .

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /go/src/{project_name}/{project_name} .
COPY scripts/entrypoint.sh .
RUN chmod +x entrypoint.sh
EXPOSE 8080
ENTRYPOINT [\"./scripts/entrypoint.sh\"]",
        project_name = project_name
    );

    return dockerfile_contents;
}


pub fn generate_makefile() -> String {
    // Define the Makefile contents
    let makefile_contents = format!(
       r#"COMPOSE_FILE=docker-compose.yml

.PHONY: all run stop

all: build run

run:
	docker-compose -f $(COMPOSE_FILE) up -d

stop:
	docker-compose -f $(COMPOSE_FILE) down

"#,);

    return makefile_contents;
}
