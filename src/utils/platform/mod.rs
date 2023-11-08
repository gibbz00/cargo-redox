pub mod architecture;
pub mod kernel;
pub mod package_manager;
pub mod processor_count;
pub mod target;

pub trait PlatformQuery: Sized {
    fn detect() -> anyhow::Result<Self>;
}
