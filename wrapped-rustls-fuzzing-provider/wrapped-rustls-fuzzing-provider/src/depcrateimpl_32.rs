// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl hash :: Context for HashContext { fn fork_finish (& self) -> hash :: Output { self . fork () . finish () } fn fork (& self) -> Box < dyn hash :: Context > { Box :: new (Self) } fn finish (self : Box < Self >) -> hash :: Output { hash :: Output :: new (HASH_OUTPUT) } fn update (& mut self , _data : & [u8]) { } }
};
}
