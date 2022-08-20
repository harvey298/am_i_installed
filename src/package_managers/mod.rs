pub mod pacman;

use anyhow::Result;

use crate::util;

use self::pacman::Pacman;

pub fn find_package(pkg_name: &str) -> Result<Package> {

    if util::get_distro()?.to_string().to_lowercase().contains("arch linux") {
        let mut pacman = Pacman::new(pkg_name.to_string().clone());
        Ok(Package { is_installed: pacman.is_installed()?, version: pacman.get_version(), pkg_name: pkg_name.to_string().clone() })

    } else { todo!("{}",util::get_distro()?) }
}

#[derive(Debug, Clone)]
pub struct Package {
    is_installed: bool,
    version: Option<String>,
    pub pkg_name: String
}

impl Package {

    pub fn is_installed(&self) -> bool {
        self.clone().is_installed
    }

    pub fn get_version(&self) -> Option<String> {
        self.clone().version
    }
}