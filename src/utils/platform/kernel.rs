use strum::EnumString;
use xshell::{cmd, Shell};

use super::PlatformQuery;

#[derive(Debug, PartialEq, EnumString)]
pub enum Kernel {
    Linux,
    FreeBSD,
    Darwin,
}

impl PlatformQuery for Kernel {
    fn detect() -> anyhow::Result<Self> {
        let shell = Shell::new()?;
        cmd!(shell, "uname --kernel-name").read()?.parse().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_kernel_name() {
        assert!(Kernel::detect().is_ok());
    }
}
