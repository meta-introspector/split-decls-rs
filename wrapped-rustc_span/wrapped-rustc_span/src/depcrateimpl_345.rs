// Generated macro for impl_345 (impl)
macro_rules! Depcrateimpl_345 {
() => {
// Module: crate
// Provides: {"impl_345"}
// Dependencies: {}
impl < 'a > FileNameDisplay < 'a > { pub fn to_string_lossy (& self) -> Cow < 'a , str > { match self . inner { FileName :: Real (inner) => inner . to_string_lossy (self . display_pref) , _ => Cow :: from (self . to_string ()) , } } }
};
}
