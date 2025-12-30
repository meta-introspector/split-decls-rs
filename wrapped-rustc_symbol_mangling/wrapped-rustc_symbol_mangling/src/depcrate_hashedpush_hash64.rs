// Generated macro for push_hash64 (function)
macro_rules! Depcrate_hashedpush_hash64 {
() => {
// Module: crate::hashed
// Provides: {"push_hash64"}
// Dependencies: {}
fn push_hash64 (hash : u64 , output : & mut String) { let hash = v0 :: encode_integer_62 (hash) ; let hash_len = hash . len () ; let _ = write ! (output , "{hash_len}H{}" , & hash [.. hash_len - 1]) ; }
};
}
