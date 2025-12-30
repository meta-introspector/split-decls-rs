// Generated macro for impl_128 (impl)
macro_rules! Depcrateimpl_128 {
() => {
// Module: crate
// Provides: {"impl_128"}
// Dependencies: {}
impl Prerelease { pub const EMPTY : Self = Prerelease { identifier : Identifier :: empty () , } ; pub fn new (text : & str) -> Result < Self , Error > { Prerelease :: from_str (text) } pub fn as_str (& self) -> & str { self . identifier . as_str () } pub fn is_empty (& self) -> bool { self . identifier . is_empty () } }
};
}
