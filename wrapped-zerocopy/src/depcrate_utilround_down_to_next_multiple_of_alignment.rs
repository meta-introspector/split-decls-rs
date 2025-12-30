// Generated macro for round_down_to_next_multiple_of_alignment (function)
macro_rules! Depcrate_utilround_down_to_next_multiple_of_alignment {
() => {
// Module: crate::util
// Provides: {"round_down_to_next_multiple_of_alignment"}
// Dependencies: {}
# [doc = " Rounds `n` down to the largest value `m` such that `m <= n` and `m % align"] # [doc = " == 0`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " May panic if `align` is not a power of two. Even if it doesn't panic in this"] # [doc = " case, it will produce nonsense results."] # [inline (always)] # [cfg_attr (kani , kani :: requires (align . is_power_of_two ()) , kani :: ensures (|& m | m <= n && m % align . get () == 0) , kani :: ensures (|& m | { m . checked_add (align . get ()) . map (| next_mul | next_mul > n) . unwrap_or (true) }))] pub (crate) const fn round_down_to_next_multiple_of_alignment (n : usize , align : NonZeroUsize ,) -> usize { # [cfg (kani)] # [kani :: proof_for_contract (round_down_to_next_multiple_of_alignment)] fn proof () { round_down_to_next_multiple_of_alignment (kani :: any () , kani :: any ()) ; } let align = align . get () ; # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] debug_assert ! (align . is_power_of_two ()) ; # [allow (clippy :: arithmetic_side_effects)] let mask = ! (align - 1) ; n & mask }
};
}
