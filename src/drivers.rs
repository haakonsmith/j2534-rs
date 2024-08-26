cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use windows::list_drivers as list;
    } else {
        pub fn list() -> io::Result<Vec<Driver>> {
            Ok(Vec::new())
        }
    }
}

/// Information about an installed PassThru driver
#[derive(Debug)]
pub struct Driver {
    pub name: String,
    pub vendor: String,
    pub path: String,
}
