// An attribute to hide warnings for unused code.
#![allow(dead_code)]
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use crate::Language;

#[derive(Debug)]
struct FileSystem {
    ptype: String,
    name: String,
    next: Vec<Option<Box<FileSystem>>>,
    content: Option<String>,
}
const FILE_TYPE: &str = "file";
const FOLDER_TYPE: &str = "folder";
const DEFAULT_FOLDER: &str = "boilerplate";

const GO_MAIN_CONTENT: &str = r#"
package main

import "fmt"
                    
func main() {
    fmt.Println("hello world!")
}
"#;
               
pub fn build_go_boilerplate(_path: &str, name: &str) {
    println!("building go boilerplate...");
    let mut filesys: HashMap<String, FileSystem> = HashMap::new();

    // construct go filesystem
    let go_filesystem = FileSystem {
        ptype: String::from(FOLDER_TYPE),
        name: String::from(name),
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
                content: Some(String::from(GO_MAIN_CONTENT)),
            })),
        ],
    };

    let folder_path  = format!("./{}_{}", Language::GO, DEFAULT_FOLDER);
    filesys.insert(folder_path.to_owned(), go_filesystem);

    // recursively crrate folders & files
    create_fs(&folder_path, filesys).unwrap();
    
}

fn create_fs(path : &String, filesys: HashMap<String, FileSystem>)-> std::io::Result<()> {
    println!("creating fs.");
    fs::create_dir_all(path)?;

    for (key, value) in filesys {
        println!("Path: {} ", key);
        for n  in value.next {
            let next = n.unwrap();
            let ptype = next.ptype;
            let content = next.content.unwrap_or_default();
            let current_path = format!("{}/{}", path, next.name);
            println!("=== folder: {} ", current_path);
            
            match ptype.as_str() {
                FOLDER_TYPE => {
                    fs::create_dir_all(current_path)?;
                },
                FILE_TYPE => {
                    let mut file = File::create(current_path).expect("Error encountered while creating file!");
                    file.write_all(content.as_bytes()).expect("Error while writing to file");
                },
                _ => {},
            }            
        }
    }
    Ok(())
}