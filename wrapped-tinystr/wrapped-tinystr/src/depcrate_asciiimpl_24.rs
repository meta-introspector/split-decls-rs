// Generated macro for impl_24 (impl)
macro_rules! Depcrate_asciiimpl_24 {
() => {
// Module: crate::ascii
// Provides: {"impl_24"}
// Dependencies: {}
impl < const N : usize > PartialEq < & str > for TinyAsciiStr < N > { fn eq (& self , other : & & str) -> bool { self . deref () == * other } }
};
}
