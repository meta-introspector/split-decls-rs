// Generated macro for impl_166 (impl)
macro_rules! Depcrate_def_idimpl_166 {
() => {
// Module: crate::def_id
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] impl Hash for DefId { fn hash < H : Hasher > (& self , h : & mut H) { (((self . krate . as_u32 () as u64) << 32) | (self . index . as_u32 () as u64)) . hash (h) } }
};
}
