// Generated macro for tests (module)
macro_rules! Depcrate_utiltests {
() => {
// Module: crate::util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_round_down_to_next_multiple_of_alignment () { fn alt_impl (n : usize , align : NonZeroUsize) -> usize { let mul = n / align . get () ; mul * align . get () } for align in [1 , 2 , 4 , 8 , 16] { for n in 0 .. 256 { let align = NonZeroUsize :: new (align) . unwrap () ; let want = alt_impl (n , align) ; let got = round_down_to_next_multiple_of_alignment (n , align) ; assert_eq ! (got , want , "round_down_to_next_multiple_of_alignment({}, {})" , n , align) ; } } } # [rustversion :: since (1.57 . 0)] # [test] # [should_panic] fn test_round_down_to_next_multiple_of_alignment_zerocopy_panic_in_const_and_vec_try_reserve () { round_down_to_next_multiple_of_alignment (0 , NonZeroUsize :: new (3) . unwrap ()) ; } }
};
}
