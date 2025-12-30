// Generated macro for impl_186 (impl)
macro_rules! Depcrate_def_idimpl_186 {
() => {
// Module: crate::def_id
// Provides: {"impl_186"}
// Dependencies: {}
impl < CTX : HashStableContext > ToStableHashKey < CTX > for DefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (* self) } }
};
}
