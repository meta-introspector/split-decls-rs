// Generated macro for impl_408 (impl)
macro_rules! Depcrate_writeimpl_408 {
() => {
// Module: crate::write
// Provides: {"impl_408"}
// Dependencies: {}
impl < W : Write > StreamWriter < W > { # [doc = " Creates an instance wrapping the provided inner writer."] pub fn new (inner : W) -> StreamWriter < W > { Self { inner , bytes_written : 0 , } } # [doc = " Consumes this wrapper, returning the underlying writer."] pub fn into_inner (self) -> W { self . inner } }
};
}
