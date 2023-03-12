// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct FileSystem {
    ptype: String,
    name: String,
    next: Vec<Option<Box<FileSystem>>>,
    content: Option<String>,
}
const FILE_TYPE: &str = "file";
const FOLDER_TYPE: &str = "file";

pub fn build_go_boilerplate(path: &str, name: &str) {
    println!("building go boilerplate...");
    let mut fs: HashMap<String, Option<FileSystem>> = HashMap::new();

    // construct go filesystem
    let go_filesystem = Some(FileSystem {
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
                content: Some(String::from("")),
            })),
        ],
    });
    fs.insert(path.to_string(), go_filesystem);

    // recursively crrate folders & files
}
