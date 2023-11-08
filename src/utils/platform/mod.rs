pub mod architecture;
pub mod kernel;
pub mod operating_system;
pub mod package_manager;
pub mod processor_count;

pub trait PlatformQuery: Sized {
    fn detect() -> anyhow::Result<Self>;
}
