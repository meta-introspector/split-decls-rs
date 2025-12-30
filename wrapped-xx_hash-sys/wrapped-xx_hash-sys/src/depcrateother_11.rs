// Generated macro for other_11 (other)
macro_rules! Depcrateother_11 {
() => {
// Module: crate
// Provides: {"other_11"}
// Dependencies: {}
extern "C" { fn XXH64 (input : * const libc :: c_void , length : libc :: size_t , seed : XXH64_hash_t) -> XXH64_hash_t ; fn XXH64_createState () -> * mut XXH64_state_t ; fn XXH64_reset (state : * mut XXH64_state_t , seed : XXH64_hash_t) -> XXH_errorcode ; fn XXH64_update (state : * mut XXH64_state_t , buffer : * const libc :: c_void , length : libc :: size_t ,) -> XXH_errorcode ; fn XXH64_digest (state : * mut XXH64_state_t) -> XXH64_hash_t ; fn XXH64_freeState (state : * mut XXH64_state_t) -> XXH_errorcode ; }
};
}
