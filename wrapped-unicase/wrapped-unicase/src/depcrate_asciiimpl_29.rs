// Generated macro for impl_29 (impl)
macro_rules! Depcrate_asciiimpl_29 {
() => {
// Module: crate::ascii
// Provides: {"impl_29"}
// Dependencies: {}
impl < S1 : AsRef < str > , S2 : AsRef < str > > PartialEq < S2 > for Ascii < S1 > { # [inline] fn eq (& self , other : & S2) -> bool { self . as_ref () . eq_ignore_ascii_case (other . as_ref ()) } }
};
}
