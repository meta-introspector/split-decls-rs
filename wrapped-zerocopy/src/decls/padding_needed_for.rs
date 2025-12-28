macro_rules! padding_needed_for {
    () => {
        # [doc = " Returns the bytes needed to pad `len` to the next multiple of `align`."] # [doc = ""] # [doc = " This function assumes that align is a power of two; there are no guarantees"] # [doc = " on the answer it gives if this is not the case."] # [cfg_attr (kani , kani :: requires (len <= isize :: MAX as usize) , kani :: requires (align . is_power_of_two ()) , kani :: ensures (|& p | (len + p) % align . get () == 0) , kani :: ensures (|& p | p < align . get ()) ,)] pub (crate) const fn padding_needed_for (len : usize , align : NonZeroUsize) -> usize { # [cfg (kani)] # [kani :: proof_for_contract (padding_needed_for)] fn proof () { padding_needed_for (kani :: any () , kani :: any ()) ; } # [allow (clippy :: arithmetic_side_effects)] let mask = align . get () - 1 ; ! (len . wrapping_sub (1)) & mask }
    };
}

padding_needed_for!();