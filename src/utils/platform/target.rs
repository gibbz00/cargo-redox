use crate::utils::platform::{architecture::Architecture, kernel::Kernel};

use super::PlatformQuery;

pub struct Target(String);

impl PlatformQuery for Target {
    fn detect() -> anyhow::Result<Self> {
        let architecture = Architecture::detect()?;

        let platform_os = match Kernel::detect()? {
            Kernel::Linux => "unknown-linux-gnu",
            Kernel::FreeBSD => "unknown-freebsd",
            Kernel::Darwin => "apple-darwin",
        };

        Ok(Self(format!("{}-{}", architecture.as_ref(), platform_os)))
    }
}
