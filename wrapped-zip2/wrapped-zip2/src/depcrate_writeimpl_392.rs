// Generated macro for impl_392 (impl)
macro_rules! Depcrate_writeimpl_392 {
() => {
// Module: crate::write
// Provides: {"impl_392"}
// Dependencies: {}
impl ZipWriterStats { fn update (& mut self , buf : & [u8]) { self . hasher . update (buf) ; self . bytes_written += buf . len () as u64 ; } }
};
}
