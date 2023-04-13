#![allow(dead_code)]
use log::info;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::tools::*;

#[derive(Debug)]
struct FileSystem {
    ptype: String,
    name: String,
    next: Vec<Option<Box<FileSystem>>>,
    content: Option<String>,
}

const TY_FILE: &str = "file";
const TY_FOLDER: &str = "folder";

fn generate_main_go(project_name: &str) -> String {
    let main_go = format!(
        r#"
package main
    
import "fmt"
                        
func main() {{
    fmt.Println("hello world! {}")
}}
"#,
        project_name
    );

    return main_go;
}

fn generate_entrypoint_sh(project_name: &str) -> String {
    let entrypoint_sh = format!(
        r#"
#!/usr/bin/env bash
set -e
echo "Buidling  {project_name}..."
go build -tags netgo -ldflags '-s -w' -o  {project_name}
echo "Exiting  {project_name}...""#,
        project_name = project_name,
    );

    return entrypoint_sh;
}

pub fn build_go_boilerplate(project_name: &str) {
    info!("building go boilerplate...");

    // construct go filesystem
    let filesystem = vec![Some(Box::new(FileSystem {
        ptype: String::from(TY_FOLDER),
        name: String::from(project_name),
        content: None,
        next: vec![
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("pkg"),
                content: None,
                next: vec![],
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("cmd"),
                content: None,
                next: vec![],
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("migrations"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("config"),
                next: vec![Some(Box::new(FileSystem {
                    ptype: String::from(TY_FILE),
                    name: String::from("config.go"),
                    next: vec![],
                    content: Some(String::from("package config")),
                }))],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("internal"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("k8s"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("build"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FOLDER),
                name: String::from("scripts"),
                next: vec![Some(Box::new(FileSystem {
                    ptype: String::from(TY_FILE),
                    name: String::from("entrypoint.sh"),
                    next: vec![],
                    content: Some(String::from(generate_entrypoint_sh(project_name))),
                }))],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FILE),
                name: String::from("main.go"),
                next: vec![],
                content: Some(String::from(generate_main_go(project_name))),
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FILE),
                name: String::from("makefile"),
                next: vec![],
                content: Some(String::from(generate_makefile())),
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FILE),
                name: String::from("Dockerfile"),
                next: vec![],
                content: Some(String::from(generate_docker_file(project_name))),
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(TY_FILE),
                name: String::from("docker-compose.yml"),
                next: vec![],
                content: Some(String::from(generate_docker_compose(project_name))),
            })),
        ],
    }))];

    let root = String::from("./");
    // recursively create folders & files
    recursive_fs(&root, filesystem).unwrap();
}

fn recursive_fs(path: &String, filesys: Vec<Option<Box<FileSystem>>>) -> std::io::Result<()> {
    for current in filesys {
        let fs = current.unwrap();
        let next = fs.next;
        let ptype = fs.ptype;
        let content = fs.content.unwrap_or_default();
        let current_path = format!("{}/{}", path, fs.name);

        match ptype.as_str() {
            TY_FOLDER => {
                info!("=== [folder]: {} ", current_path);
                fs::create_dir_all(current_path.clone())?;
            }
            TY_FILE => {
                info!("=== [file]: {} ", current_path);
                let mut file = File::create(current_path.clone())
                    .expect("Error encountered while creating file!");
                file.write_all(content.as_bytes())
                    .expect("Error while writing to file");
            }
            _ => {}
        }

        recursive_fs(&current_path, next).unwrap()
    }
    Ok(())
}
