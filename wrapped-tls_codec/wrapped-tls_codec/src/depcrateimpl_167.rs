// Generated macro for impl_167 (impl)
macro_rules! Depcrateimpl_167 {
() => {
// Module: crate
// Provides: {"impl_167"}
// Dependencies: {}
impl From < U24 > for usize { fn from (value : U24) -> usize { const LEN : usize = core :: mem :: size_of :: < usize > () ; let mut usize_bytes = [0u8 ; LEN] ; usize_bytes [LEN - 3 ..] . copy_from_slice (& value . 0) ; usize :: from_be_bytes (usize_bytes) } }
};
}
