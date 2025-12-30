// Generated macro for impl_178 (impl)
macro_rules! Depcrate_def_idimpl_178 {
() => {
// Module: crate::def_id
// Provides: {"impl_178"}
// Dependencies: {}
impl LocalDefId { # [inline] pub fn to_def_id (self) -> DefId { DefId { krate : LOCAL_CRATE , index : self . local_def_index } } # [inline] pub fn is_top_level_module (self) -> bool { self == CRATE_DEF_ID } }
};
}
