// Generated macro for impl_100 (impl)
macro_rules! Depcrate_directoryimpl_100 {
() => {
// Module: crate::directory
// Provides: {"impl_100"}
// Dependencies: {}
impl From < OsString > for Directory { fn from (os_string : OsString) -> Self { Directory :: new (PathBuf :: from (os_string)) } }
};
}
