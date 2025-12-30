// Generated macro for impl_87 (impl)
macro_rules! Depcrate_sized_chunkimpl_87 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_87"}
// Dependencies: {}
impl < A , const N : usize > Clone for Chunk < A , N > where A : Clone , { fn clone (& self) -> Self { let mut out = Self :: new () ; out . left = self . left ; out . right = self . left ; for index in self . left .. self . right { unsafe { Chunk :: force_write (index , (* self . ptr (index)) . clone () , & mut out) } out . right = index + 1 ; } out } }
};
}
