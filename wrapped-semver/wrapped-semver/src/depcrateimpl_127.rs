// Generated macro for impl_127 (impl)
macro_rules! Depcrateimpl_127 {
() => {
// Module: crate
// Provides: {"impl_127"}
// Dependencies: {}
impl Comparator { pub fn parse (text : & str) -> Result < Self , Error > { Comparator :: from_str (text) } pub fn matches (& self , version : & Version) -> bool { eval :: matches_comparator (self , version) } }
};
}
