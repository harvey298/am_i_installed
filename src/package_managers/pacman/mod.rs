
use std::process::Command;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Pacman {
    is_installed: Option<bool>,
    version: Option<String>,
    pkg_name: String
}

impl Pacman {
    pub fn new(pkg_name: String) -> Self {
        Self{ is_installed: None, version: None, pkg_name: pkg_name }
    }
    
    pub fn is_installed(&mut self) -> Result<bool> {

        match self.is_installed {
            Some(o) => Ok(o),
            None => {
                // Search for Package
                let search: String = String::from_utf8(
                    Command::new("pacman")
                        .arg("-Q")
                        .arg(self.clone().pkg_name)
                        .output()
                        .expect("failed to run Pacman")
                        .stdout
                )?;
        
                // error: package '{pkgname}' was not found
                let no_pkg_response = format!("error: package '{}' was not found",self.clone().pkg_name);
        
                if search.clone().contains(&no_pkg_response) {
                    self.is_installed = Some(false);
                    
                } else { 
                    self.is_installed = Some(true);
                    self.version = Some(search.replace(&format!("{} ",self.clone().pkg_name),"").replace("\n",""));
                }
        
        
                Ok(self.is_installed.unwrap_or(false))
            },
        }
    }

    pub fn get_version(&self) -> Option<String> {
        self.clone().version
    }
}

