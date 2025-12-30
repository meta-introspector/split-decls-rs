// Generated macro for impl_29 (impl)
macro_rules! Depcrate_hstringimpl_29 {
() => {
// Module: crate::hstring
// Provides: {"impl_29"}
// Dependencies: {}
impl Deref for HSTRING { type Target = [u16] ; fn deref (& self) -> & [u16] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts (header . data , header . len as usize) } } else { const EMPTY : [u16 ; 1] = [0] ; & EMPTY [.. 0] } } }
};
}
