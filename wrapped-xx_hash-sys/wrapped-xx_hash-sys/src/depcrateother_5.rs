// Generated macro for other_5 (other)
macro_rules! Depcrateother_5 {
() => {
// Module: crate
// Provides: {"other_5"}
// Dependencies: {}
extern "C" { fn XXH32 (input : * const libc :: c_void , length : libc :: size_t , seed : XXH32_hash_t) -> XXH32_hash_t ; fn XXH32_createState () -> * mut XXH32_state_t ; fn XXH32_reset (state : * mut XXH32_state_t , seed : XXH32_hash_t) -> XXH_errorcode ; fn XXH32_update (state : * mut XXH32_state_t , buffer : * const libc :: c_void , length : libc :: size_t ,) -> XXH_errorcode ; fn XXH32_digest (state : * mut XXH32_state_t) -> XXH32_hash_t ; fn XXH32_freeState (state : * mut XXH32_state_t) -> XXH_errorcode ; }
};
}
