use super::{package_manager::PackageManager, PlatformQuery};

pub enum OperatingSystem {
    Suse,
    Debian,
    Fedora,
    Gentoo,
    Solus,
    Arch,
    FreeBSD,
    MacOS,
}

impl PlatformQuery for OperatingSystem {
    fn detect() -> anyhow::Result<Self> {
        PackageManager::detect().map(Into::into)
    }
}

impl From<PackageManager> for OperatingSystem {
    fn from(package_manager: PackageManager) -> Self {
        match package_manager {
            PackageManager::Zypper => OperatingSystem::Suse,
            PackageManager::Apt => OperatingSystem::Debian,
            PackageManager::Dnf => OperatingSystem::Fedora,
            PackageManager::Portage => OperatingSystem::Gentoo,
            PackageManager::Eopkg => OperatingSystem::Solus,
            PackageManager::Pacman => OperatingSystem::Arch,
            PackageManager::Pkg => OperatingSystem::FreeBSD,
            PackageManager::Homebrew => OperatingSystem::MacOS,
        }
    }
}
