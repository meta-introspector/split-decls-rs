// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
impl Name { # [doc = " Create a new name."] pub fn new (name : impl AsRef < str >) -> Self { Self (name . as_ref () . into ()) } # [doc = " Get the name as a string."] # [must_use] pub fn as_str (& self) -> & str { & self . 0 } }
};
}
