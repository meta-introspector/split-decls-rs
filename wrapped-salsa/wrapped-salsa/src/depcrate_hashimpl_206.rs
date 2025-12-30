// Generated macro for impl_206 (impl)
macro_rules! Depcrate_hashimpl_206 {
() => {
// Module: crate::hash
// Provides: {"impl_206"}
// Dependencies: {}
impl Hasher for TypeIdHasher { fn write (& mut self , _ : & [u8]) { unreachable ! ("`TypeId` calls `write_u64`") ; } # [inline] fn write_u64 (& mut self , id : u64) { self . 0 = id ; } # [inline] fn finish (& self) -> u64 { self . 0 } }
};
}
