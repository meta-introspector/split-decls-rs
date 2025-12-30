// Generated macro for benchmarks (module)
macro_rules! Depcrate_crypto_tls12benchmarks {
() => {
// Module: crate::crypto::tls12
// Provides: {"benchmarks"}
// Dependencies: {}
# [cfg (all (bench , feature = "ring"))] mod benchmarks { # [bench] fn bench_sha256 (b : & mut test :: Bencher) { use crate :: crypto :: hmac :: Hmac ; use crate :: crypto :: ring :: hmac ; let label = & b"extended master secret" [..] ; let seed = [0u8 ; 32] ; let key = & b"secret" [..] ; b . iter (| | { let mut out = [0u8 ; 48] ; super :: prf (& mut out , & * hmac :: HMAC_SHA256 . with_key (key) , & label , & seed) ; test :: black_box (out) ; }) ; } }
};
}
