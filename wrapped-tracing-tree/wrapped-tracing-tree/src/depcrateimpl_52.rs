// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl Data { pub fn new (attrs : & Attributes < '_ > , written : bool) -> Self { let mut span = Self { start : Instant :: now () , kvs : Vec :: new () , written , } ; attrs . record (& mut span) ; span } }
};
}
