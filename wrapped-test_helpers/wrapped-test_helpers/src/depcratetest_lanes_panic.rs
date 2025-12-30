// Generated macro for test_lanes_panic (macro)
macro_rules! Depcratetest_lanes_panic {
() => {
// Module: crate
// Provides: {"test_lanes_panic"}
// Dependencies: {}
# [doc = " Expand a const-generic `#[should_panic]` test into separate tests for each possible lane count."] # [macro_export] macro_rules ! test_lanes_panic { { $ (fn $ test : ident < const $ lanes : ident : usize > () $ body : tt) * } => { $ (mod $ test { use super ::*; fn implementation < const $ lanes : usize > () where core_simd :: simd :: LaneCount <$ lanes >: core_simd :: simd :: SupportedLaneCount , $ body $ crate :: test_lanes_helper ! (# [should_panic] ; lanes_1 1 ; lanes_2 2 ; lanes_3 3 ; lanes_6 6 ;) ; # [cfg (not (miri))] $ crate :: test_lanes_helper ! (# [should_panic] ; lanes_4 4 ; lanes_5 5 ; lanes_7 7 ; lanes_8 8 ; lanes_9 9 ; lanes_10 10 ; lanes_11 11 ; lanes_12 12 ; lanes_13 13 ; lanes_14 14 ; lanes_15 15 ; lanes_16 16 ; lanes_17 17 ; lanes_24 24 ; lanes_32 32 ; lanes_47 47 ; lanes_56 56 ; lanes_57 57 ; lanes_63 63 ; lanes_64 64 ;) ; }) * } }
};
}
