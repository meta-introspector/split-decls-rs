// Generated macro for impl_590 (impl)
macro_rules! Depcrate_io_nostdimpl_590 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_590"}
// Dependencies: {}
impl < R : Read > Take < R > { pub fn limit (& self) -> u64 { self . limit } pub fn set_limit (& mut self , limit : u64) { self . limit = limit ; } pub fn get_ref (& self) -> & R { & self . inner } pub fn get_mut (& mut self) -> & mut R { & mut self . inner } pub fn into_inner (self) -> R { self . inner } }
};
}
