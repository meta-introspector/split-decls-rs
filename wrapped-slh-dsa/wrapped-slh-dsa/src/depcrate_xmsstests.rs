// Generated macro for tests (module)
macro_rules! Depcrate_xmsstests {
() => {
// Module: crate::xmss
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: PkSeed ; use crate :: SkSeed ; use crate :: util :: macros :: test_parameter_sets ; use hex_literal :: hex ; use hybrid_array :: Array ; use rand :: Rng ; use rand :: RngCore ; use rand :: rng ; use typenum :: Unsigned ; use crate :: { address :: WotsHash , hashes :: Shake128f , xmss :: XmssParams } ; # [test] fn test_xmss_node_shake128f_kat () { let sk_seed = SkSeed (Array ([1 ; 16])) ; let pk_seed = PkSeed (Array ([2 ; 16])) ; let adrs = WotsHash :: default () ; let node = Shake128f :: xmss_node (& sk_seed , 0 , < Shake128f as XmssParams > :: HPrime :: U32 , & pk_seed , & adrs ,) ; let expected = hex ! ("94e24679fb2460b97332db131c38bec9") ; assert_eq ! (node . as_slice () , expected) ; } # [test] # [cfg (feature = "alloc")] fn test_sign_shake128f_kat () { let sk_seed = SkSeed (Array ([1 ; 16])) ; let pk_seed = PkSeed (Array ([2 ; 16])) ; let adrs = WotsHash :: default () ; let m = Array ([3 ; 16]) ; let idx = 3 ; let sig = Shake128f :: xmss_sign (& m , & sk_seed , & pk_seed , idx , & adrs) ; let expected = hex ! ("
        a77a0b07e558b023f653a954d886ac66ded67b313f9db7fd93da00686be66a3f
        2e2d3e841292bf5a4060d88509e9a2a51e0bbae6835482bceabce76c5653546d
        08c2f5f78e7491f755f35380d965598891131bdd4c57df2397eed8062a1038fb
        10c758bb30c6ea3859db4eb6296269d170d86cc67804dc63a61e5f30af709aad
        2407624eb81549e87c326c2a646c2b995dfad81cc007286b6f50b56f61352fa2
        752a30aa4f63cc367a7a1c57140a086cc43387ce5f530d84538d0c503d051be2
        9c0040486c2953d34e3817bfcb6f198e545476ddd93930af48333b4e7e0eba03
        3bdbc1badca23875d2f4345699075558a68c8f53865c0b2151208a7a5a4b0c7d
        270b71d5688c6d727525e3fd9c75b9656e13394777faee925fe8cda6e2b7c52a
        684f218679a48b942127f89ffaa069db21659a09266e9304ce870c16094bf585
        6ed93c0748b9479a95d4309c74c2da26b2cf2e5f2090f02601b80c3373b14666
        f0bd973d10c7eb649966d1ffd3e87979899812fef1e23f5703a99924001d9ba9
        522ea93575ad20143eeeeff77b8d192870932b1583459271f634a65441fe1907
        370f71e4d9312b930a66e1b85cba8f4a404c703c7c38ada5c6b95824c2c0ff87
        b1e3f258189d949430c516d2c2192ffbb8d687b10228d7ecf47f86c1299825a8
        b6ee7c560f4bd1720aabdca41c8a5569e9917f906efca17d5f080e65e5a16386
        c9bb4f1ad49404340df212e94d77ff5a25b8649b725e1993dc66f37a89058499
        107bb57a4f699688406e89a44776b95bd1af01290496fb4f3abba58eb407eff9
        c1dfd1362d169170f8b7364c6aa8e6507f049484e5d9b934e86d61b1d3155b5a") ; assert_eq ! (sig . to_vec () , expected) ; } fn test_sign_verify < Xmss : XmssParams > () { let mut rng = rng () ; let sk_seed = SkSeed :: new (& mut rng) ; let pk_seed = PkSeed :: new (& mut rng) ; let mut msg = Array :: < u8 , _ > :: default () ; rng . fill_bytes (msg . as_mut_slice ()) ; let idx = rng . random_range (0 .. (1 << Xmss :: HPrime :: U32)) ; let adrs = WotsHash :: default () ; let pk = Xmss :: xmss_node (& sk_seed , 0 , Xmss :: HPrime :: U32 , & pk_seed , & adrs) ; let sig = Xmss :: xmss_sign (& msg , & sk_seed , & pk_seed , idx , & adrs) ; let pk_recovered = Xmss :: xmss_pk_from_sig (idx , & sig , & msg , & pk_seed , & adrs) ; assert_eq ! (pk , pk_recovered) ; } test_parameter_sets ! (test_sign_verify) ; fn test_sign_verify_fail < Xmss : XmssParams > () { let mut rng = rng () ; let sk_seed = SkSeed :: new (& mut rng) ; let pk_seed = PkSeed :: new (& mut rng) ; let mut msg = Array :: < u8 , _ > :: default () ; rng . fill_bytes (msg . as_mut_slice ()) ; let idx = rng . random_range (0 .. (1 << Xmss :: HPrime :: U32)) ; let adrs = WotsHash :: default () ; let pk = Xmss :: xmss_node (& sk_seed , 0 , Xmss :: HPrime :: U32 , & pk_seed , & adrs) ; let sig = Xmss :: xmss_sign (& msg , & sk_seed , & pk_seed , idx , & adrs) ; msg [0] ^= 0xff ; let pk_recovered = Xmss :: xmss_pk_from_sig (idx , & sig , & msg , & pk_seed , & adrs) ; assert_ne ! (pk , pk_recovered) ; } test_parameter_sets ! (test_sign_verify_fail) ; }
};
}
