// Generated macro for tests (module)
macro_rules! Depcrate_hashestests {
() => {
// Module: crate::hashes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use hex_literal :: hex ; fn prf_msg < H : HashSuite > (expected : & [u8]) { let sk_prf = SkPrf (Array :: < u8 , H :: N > :: from_fn (| _ | 0)) ; let opt_rand = Array :: < u8 , H :: N > :: from_fn (| _ | 1) ; let msg = [2u8 ; 32] ; let result = H :: prf_msg (& sk_prf , & opt_rand , & [& [msg]]) ; assert_eq ! (result . as_slice () , expected) ; } fn h_msg < H : HashSuite > (expected : & [u8]) { let rand = Array :: < u8 , H :: N > :: from_fn (| _ | 0) ; let pk_seed = PkSeed (Array :: < u8 , H :: N > :: from_fn (| _ | 1)) ; let pk_root = Array :: < u8 , H :: N > :: from_fn (| _ | 2) ; let msg = [3u8 ; 32] ; let result = H :: h_msg (& rand , & pk_seed , & pk_root , & [& [msg]]) ; assert_eq ! (result . as_slice () , expected) ; } # [test] fn prf_msg_shake128f () { prf_msg :: < Shake128f > (& hex ! ("bc5c062307df0a41aeeae19ad655f7b2")) ; } # [test] fn prf_msg_sha2_128_f () { prf_msg :: < Sha2_128f > (& hex ! ("6a4b5cf23911d4f3a6591d7003445316")) ; } # [test] fn h_msg_sha2_128_f () { h_msg :: < Sha2_128f > (& hex ! ("56658221f675d907a309255e8faef639d11e6a1118fa05d3bbd26179a7e0a54a7f5b")) ; } # [test] fn h_msg_sha2_256_f () { h_msg :: < Sha2_256f > (& hex ! ("8c86dfb66392d1b647df0deab90be68fb6f988513e84d3ef75fa68591122bb5d74f6413672db5164e56492b7ca2c2e0335")) ; } }
};
}
