use strum::{AsRefStr, EnumIter, IntoEnumIterator};

use super::PlatformQuery;

#[derive(EnumIter, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum PackageManager {
    Zypper,
    Apt,
    Dnf,
    Portage,
    Eopkg,
    Pacman,
    Pkg,
    Homebrew,
}

impl PlatformQuery for PackageManager {
    fn detect() -> anyhow::Result<Self> {
        for package_manager in PackageManager::iter() {
            if which::which(package_manager.as_ref()).is_ok() {
                return Ok(package_manager);
            }
        }

        anyhow::bail!("Unable to select system package manager.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_package_manager() {
        assert!(PackageManager::detect().is_ok())
    }
}
