// Generated macro for impl_273 (impl)
macro_rules! Depcrate_symbolimpl_273 {
() => {
// Module: crate::symbol
// Provides: {"impl_273"}
// Dependencies: {}
impl < CTX > ToStableHashKey < CTX > for Symbol { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> String { self . as_str () . to_string () } }
};
}
