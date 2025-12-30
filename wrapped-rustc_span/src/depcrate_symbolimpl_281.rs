// Generated macro for impl_281 (impl)
macro_rules! Depcrate_symbolimpl_281 {
() => {
// Module: crate::symbol
// Provides: {"impl_281"}
// Dependencies: {}
impl < CTX > ToStableHashKey < CTX > for Symbol { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> String { self . as_str () . to_string () } }
};
}
