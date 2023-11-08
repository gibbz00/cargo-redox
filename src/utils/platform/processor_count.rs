use super::PlatformQuery;

pub struct ProcessorCount(usize);

impl PlatformQuery for ProcessorCount {
    fn detect() -> anyhow::Result<Self> {
        Ok(Self(num_cpus::get()))
    }
}
