// Generated macro for tests (module)
macro_rules! Depcrate_wotstests {
() => {
// Module: crate::wots
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { PkSeed , SkSeed , util :: macros :: test_parameter_sets } ; use hex_literal :: hex ; use hybrid_array :: Array ; use rand :: { RngCore , rng } ; use crate :: { address :: WotsHash , hashes :: Shake128f } ; use super :: WotsParams ; fn test_sign_verify < Wots : WotsParams > () { let mut rng = rng () ; let sk_seed = SkSeed :: new (& mut rng) ; let pk_seed = PkSeed :: new (& mut rng) ; let mut msg = Array :: < u8 , _ > :: default () ; rng . fill_bytes (msg . as_mut_slice ()) ; let adrs = & WotsHash :: default () ; let pk = Wots :: wots_pk_gen (& sk_seed , & pk_seed , adrs) ; let sig = Wots :: wots_sign (& msg , & sk_seed , & pk_seed , adrs) ; let pk_recovered = Wots :: wots_pk_from_sig (& sig , & msg , & pk_seed , adrs) ; assert_eq ! (pk , pk_recovered) ; } test_parameter_sets ! (test_sign_verify) ; fn test_sign_verify_fail < Wots : WotsParams > () { let mut rng = rng () ; let sk_seed = SkSeed :: new (& mut rng) ; let pk_seed = PkSeed :: new (& mut rng) ; let mut msg = Array :: < u8 , _ > :: default () ; rng . fill_bytes (msg . as_mut_slice ()) ; let adrs = & WotsHash :: default () ; let pk = Wots :: wots_pk_gen (& sk_seed , & pk_seed , adrs) ; let sig = Wots :: wots_sign (& msg , & sk_seed , & pk_seed , adrs) ; msg [0] ^= 0xff ; let pk_recovered = Wots :: wots_pk_from_sig (& sig , & msg , & pk_seed , adrs) ; assert_ne ! (pk , pk_recovered , "Signature verification should fail with a modified message") ; } test_parameter_sets ! (test_sign_verify_fail) ; # [test] fn test_pk_gen_shake128f_kat () { let sk_seed = SkSeed (Array ([1 ; 16])) ; let pk_seed = PkSeed (Array ([2 ; 16])) ; let adrs = WotsHash :: default () ; let expected = Array (hex ! ("98b63dd1574484876b1f8a1120421eac")) ; let result = Shake128f :: wots_pk_gen (& sk_seed , & pk_seed , & adrs) ; assert_eq ! (result , expected) ; } # [test] # [cfg (feature = "alloc")] fn test_sign_shake128f_kat () { let sk_seed = SkSeed (Array ([1 ; 16])) ; let pk_seed = PkSeed (Array ([2 ; 16])) ; let adrs = & WotsHash :: default () ; let msg = Array ([3 ; 16]) ; let expected = & hex ! ("f7bcb9575590faae2e6a8ae33149082d2ec777cff4051f43177ef44bcbd2c18d
            a94146c50037c914461dd6ed720192b059bd2be6ed8d8cf26e4e9d68fbf9ded1
            6c334bed21677c6a3679f17a8425de40431b4317326c5d825d931b4a54a1b81f
            e7ad259086ea665109a7eca79f03e3619d99af5d0419fece8300973f29467f28
            d2b18639eeaa826488f6c785d492703463e80f8b088e64de9ca3b373cead611f
            d356bf6c22f70f98f229174a9ac815342f0439eb289a78f49f47aa8c3f272a15
            f5f0f5020b5d71981254daa9e1f01a90248935c1c67ad1cf71d9224184820cf9
            ece9b737ec986c86ba0a9431ff8485c274140bebc9d856316d49128eb075f81a
            c00d32b9f949940f2dd684a2e615e16b47093eb49e3bc9d77e69c7944d7063c6
            f8b4b5aa46fe759999fa2892ce4c7881b80f38d684427a0b77f3ad43377833d2
            d94c600b340ea408a0ad7c32c409bdb4ebaade3b1dda4ac8584acba979c845a9
            b0ddfc69ea22ffb415745b779b45d7af00ca9fde87e5d59385d7b5cedec6e30f
            3346f573f59a00af993a2ec314ed951e3a8c00f69364a82fa34d14933fe3cdb7
            bd5e5d511297695bad5cda22daea8d39f61d4ed34412acd1f5399a54953ae04b
            09828f90877ad7f01605631ace0a4e7c773cc887e2d0fa0bd3d6db811794df3a
            a8721c308482ccb511c9133311653ce8f9c2336e2980c2ab554c41bad436c0c7
            1c394d3f7eafcea2806c153113d6291a912c0e73e44197763b9ead341c298585
            bc6e16d8458fc1917ff4ac57de461ee1") ; let result = Shake128f :: wots_sign (& msg , & sk_seed , & pk_seed , adrs) ; assert_eq ! (result . to_vec () , expected . as_slice ()) ; } }
};
}
