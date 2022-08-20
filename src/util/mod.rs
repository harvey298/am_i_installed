extern crate lazy_static;

use lazy_static::lazy_static;
use anyhow::Result;
use std::{fs, path::Path};

lazy_static! {
    static ref VERSION_FILES: Vec<String> = get_possible_name_file().unwrap();
}

pub fn get_distro() -> Result<String> {
    
    for vf in &*VERSION_FILES {
        if Path::new(&vf.clone()).exists() {
            let data = fs::read_to_string(vf)?;
            if data != "" {
                return Ok(data)
            }
        }
    }

    Ok("".to_string())
}


fn get_possible_name_file() -> Result<Vec<String>> {

    let paths = fs::read_dir("/etc/").unwrap();

    let mut files = Vec::new();

    if Path::new("/etc/issue").exists() {
        files.push("/etc/issue".to_string())
    }

    for path in paths {
        let p = path?.path();
        let p = p.to_str().unwrap_or("");

        if p.contains("release") {
            files.push(p.to_string())
        }
    }

    Ok(files)
}