// Generated macro for tests (module)
macro_rules! Depcrate_legacy_reducetests {
() => {
// Module: crate::legacy::reduce
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: hwexpand ; use crate :: legacy :: reduce :: { follower_idx_bitlen , lsb , max_dist } ; const HAMLET_2048 : & [u8 ; 1285] = include_bytes ! ("../../tests/data/legacy/reduce_hamlet_2048.bin") ; const HAMLET_2048_OUT : & [u8 ; 2048] = include_bytes ! ("../../tests/data/legacy/implode_hamlet_2048.out") ; # [test] fn test_lsb () { assert_eq ! (lsb (0xFF , 8) , 0xFF) ; for i in 0 .. 7 { assert_eq ! (lsb (0xFF , i) , (1 << i) - 1) ; } } # [test] fn test_expand_hamlet2048 () { let mut dst = Vec :: new () ; hwexpand (HAMLET_2048 , 2048 , 4 , & mut dst) . unwrap () ; assert_eq ! (dst . len () , 2048) ; assert_eq ! (& dst , & HAMLET_2048_OUT) ; } const ZEROS_REDUCED : & [u8 ; 1297] = include_bytes ! ("../../tests/data/legacy/reduce_zero_reduced.bin") ; # [test] fn test_expand_zeros () { let mut dst = Vec :: new () ; hwexpand (ZEROS_REDUCED , 2048 + 1024 , 4 , & mut dst) . unwrap () ; assert_eq ! (dst . len () , 2048 + 1024) ; for i in 0 .. (1 << 10) { assert_eq ! (dst [(1 << 11) + i] , 0) ; } } fn orig_follower_idx_bitlen (n : u8) -> u8 { if n > 16 { return 5 ; } if n > 8 { return 4 ; } if n > 4 { return 3 ; } if n > 2 { return 2 ; } if n > 0 { return 1 ; } 0 } # [test] fn test_follower_idx_biten () { for i in 0 ..= 32 { assert_eq ! (orig_follower_idx_bitlen (i) , follower_idx_bitlen (i)) ; } } # [test] fn test_max_dist () { for i in 1 ..= 4 { let v_dist_bits = i as usize ; let c = 1 << (v_dist_bits + 8) ; assert_eq ! (max_dist (i) , c) ; } } }
};
}
