// Generated macro for impl_188 (impl)
macro_rules! Depcrate_def_idimpl_188 {
() => {
// Module: crate::def_id
// Provides: {"impl_188"}
// Dependencies: {}
impl < CTX : HashStableContext > ToStableHashKey < CTX > for CrateNum { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { self . as_def_id () . to_stable_hash_key (hcx) } }
};
}
