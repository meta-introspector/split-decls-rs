use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl OsVersion {
    /// Creates a new `OsVersion` with the given values.
    pub const fn new(major: u32, minor: u32, pack: u32, build: u32) -> Self {
        Self { major, minor, pack, build }
    }
    /// Gets the version information of the currently running operating system.
    #[cfg(not(test))]
    pub fn current() -> Self {
        let mut info = OSVERSIONINFOEXW::new();
        unsafe {
            RtlGetVersion(&mut info as *mut _ as *mut _);
        }
        Self {
            major: info.dwMajorVersion,
            minor: info.dwMinorVersion,
            pack: info.wServicePackMajor as u32,
            build: info.dwBuildNumber,
        }
    }
    /// Hook used for testing `ge`.
    #[cfg(test)]
    fn current() -> Self {
        test::test_current()
    }
}
