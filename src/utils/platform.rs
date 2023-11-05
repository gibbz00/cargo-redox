use strum::EnumString;
use xshell::{cmd, Shell};

#[derive(Debug, PartialEq, EnumString)]
pub enum KernelName {
    Linux,
    FreeBSD,
}

pub trait PlatformQuery {
    fn kernel_name(&self) -> anyhow::Result<KernelName>;
}

impl PlatformQuery for Shell {
    fn kernel_name(&self) -> anyhow::Result<KernelName> {
        let uname_string = cmd!(self, "uname").read()?;
        uname_string.parse::<KernelName>().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn gets_linux_kernel_name() -> anyhow::Result<()> {
        assert_eq!(KernelName::Linux, Shell::new()?.kernel_name()?);
        Ok(())
    }

    #[test]
    #[cfg(target_os = "freebsd")]
    fn gets_free_bsd_kernel_name() -> anyhow::Result<()> {
        assert_eq!(KernelName::FreeBSD, Shell::new()?.kernel_name()?);
        Ok(())
    }
}
