use strum::{AsRefStr, EnumString};
use xshell::{cmd, Shell};

use super::PlatformQuery;

#[derive(EnumString, AsRefStr)]
pub enum Architecture {
    #[allow(non_camel_case_types)]
    aarch64,
    #[allow(non_camel_case_types)]
    x86_64,
    #[allow(non_camel_case_types)]
    i386,
}

impl PlatformQuery for Architecture {
    fn detect() -> anyhow::Result<Self> {
        let shell = Shell::new()?;
        cmd!(shell, "uname --machine").read()?.parse::<Architecture>().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_architecture() {
        assert!(Architecture::detect().is_ok())
    }
}
