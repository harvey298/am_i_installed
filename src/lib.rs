#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result};

    #[test]
    fn find_package_pacman() -> Result<()> {
        let mut pkg = package_managers::pacman::Pacman::new("linux".to_string());
        println!("is installed: {}",pkg.is_installed()?);
        println!("version: {:?}",pkg.get_version());
        Ok(())
    }

    #[test]
    fn find_package() -> Result<()> {
        let pkg = package_managers::find_package("linux")?;
        println!("is installed: {}",pkg.is_installed());
        println!("version: {:?}",pkg.get_version());
        Ok(())
    }

    #[test]
    fn get_distro() -> Result<()> {
        println!("Distro: {}",util::get_distro()?);
        Ok(())
    }
}

mod util;
pub mod package_managers;