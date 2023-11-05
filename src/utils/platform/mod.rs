pub mod architecture;
pub mod kernel;
pub mod operating_system;
pub mod package_manager;

pub trait PlatformQuery: Sized {
    fn detect() -> anyhow::Result<Self>;
}
