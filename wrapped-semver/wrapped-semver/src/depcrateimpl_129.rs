// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl BuildMetadata { pub const EMPTY : Self = BuildMetadata { identifier : Identifier :: empty () , } ; pub fn new (text : & str) -> Result < Self , Error > { BuildMetadata :: from_str (text) } pub fn as_str (& self) -> & str { self . identifier . as_str () } pub fn is_empty (& self) -> bool { self . identifier . is_empty () } }
};
}
