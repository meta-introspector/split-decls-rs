// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl PartialEq < SmolStr > for SmolStr { fn eq (& self , other : & SmolStr) -> bool { self . 0 . ptr_eq (& other . 0) || self . as_str () == other . as_str () } }
};
}
