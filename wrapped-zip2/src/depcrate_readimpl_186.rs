// Generated macro for impl_186 (impl)
macro_rules! Depcrate_readimpl_186 {
() => {
// Module: crate::read
// Provides: {"impl_186"}
// Dependencies: {}
impl < R : Read > HasZipMetadata for ZipFile < '_ , R > { fn get_metadata (& self) -> & ZipFileData { self . data . as_ref () } }
};
}
