#![allow(dead_code)]
use log::info;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct FileSystem {
    ptype: String,
    name: String,
    next: Vec<Option<Box<FileSystem>>>,
    content: Option<String>,
}

const FILE_TYPE: &str = "file";
const FOLDER_TYPE: &str = "folder";

pub fn build_go_boilerplate(project_name: &str) {
    info!("building go boilerplate...");
    let mut filesys: HashMap<String, FileSystem> = HashMap::new();

    let content: &str = r#"
package main

import "fmt"
                    
func main() {
    fmt.Println("hello world!")
}
"#;

    // construct go filesystem
    let go_filesystem = FileSystem {
        ptype: String::from(FOLDER_TYPE),
        name: String::from(project_name),
        content: None,
        next: vec![
            Some(Box::new(FileSystem {
                ptype: String::from(FOLDER_TYPE),
                name: String::from("pkg"),
                content: None,
                next: vec![],
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(FOLDER_TYPE),
                name: String::from("cmd"),
                content: None,
                next: vec![],
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(FOLDER_TYPE),
                name: String::from("migrations"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(FOLDER_TYPE),
                name: String::from("internal"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(FOLDER_TYPE),
                name: String::from("k8s"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(FOLDER_TYPE),
                name: String::from("build"),
                next: vec![],
                content: None,
            })),
            Some(Box::new(FileSystem {
                ptype: String::from(FILE_TYPE),
                name: String::from("main.go"),
                next: vec![],
                content: Some(String::from(content)),
            })),
        ],
    };

    let folder_path = format!("./{}", project_name);
    filesys.insert(folder_path.to_owned(), go_filesystem);

    // recursively create folders & files
    create_fs(&folder_path, filesys).unwrap();
}

fn create_fs(path: &String, filesys: HashMap<String, FileSystem>) -> std::io::Result<()> {
    fs::create_dir_all(path)?;

    for (_key, value) in filesys {
        for n in value.next {
            let next = n.unwrap();
            let ptype = next.ptype;
            let content = next.content.unwrap_or_default();
            let current_path = format!("{}/{}", path, next.name);

            match ptype.as_str() {
                FOLDER_TYPE => {
                    info!("=== [folder]: {} ", current_path);
                    fs::create_dir_all(current_path)?;
                }
                FILE_TYPE => {
                    info!("=== [file]: {} ", current_path);
                    let mut file =
                        File::create(current_path).expect("Error encountered while creating file!");
                    file.write_all(content.as_bytes())
                        .expect("Error while writing to file");
                }
                _ => {}
            }
        }
    }
    Ok(())
}
