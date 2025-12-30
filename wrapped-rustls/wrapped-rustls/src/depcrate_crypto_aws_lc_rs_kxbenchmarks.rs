// Generated macro for benchmarks (module)
macro_rules! Depcrate_crypto_aws_lc_rs_kxbenchmarks {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"benchmarks"}
// Dependencies: {}
# [cfg (bench)] mod benchmarks { # [bench] fn bench_x25519 (b : & mut test :: Bencher) { bench_any (b , super :: X25519) ; } # [bench] fn bench_ecdh_p256 (b : & mut test :: Bencher) { bench_any (b , super :: SECP256R1) ; } # [bench] fn bench_ecdh_p384 (b : & mut test :: Bencher) { bench_any (b , super :: SECP384R1) ; } fn bench_any (b : & mut test :: Bencher , kxg : & dyn super :: SupportedKxGroup) { b . iter (| | { let akx = kxg . start () . unwrap () . into_single () ; let pub_key = akx . pub_key () . to_vec () ; test :: black_box (akx . complete (& pub_key) . unwrap ()) ; }) ; } }
};
}
