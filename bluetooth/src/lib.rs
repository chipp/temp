#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::BLEAdvertisementObserver;

pub struct ServiceData(pub Vec<u8>);
