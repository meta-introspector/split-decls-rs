// Generated macro for impl_521 (impl)
macro_rules! Depcrate_io_nostdimpl_521 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_521"}
// Dependencies: {}
impl < R : Read > Read for Take < R > { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > { if self . limit == 0 { return Ok (0) ; } let at_most = (self . limit as usize) . min (buf . len ()) ; let bytes = self . inner . read (& mut buf [.. at_most]) ? ; self . limit -= bytes as u64 ; Ok (bytes) } }
};
}
