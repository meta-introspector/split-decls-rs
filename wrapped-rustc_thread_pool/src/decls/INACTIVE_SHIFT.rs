macro_rules! INACTIVE_SHIFT {
    () => {
        # [doc = " Bits to shift to select the inactive threads"] # [doc = " (used with `select_bits`)."] # [allow (clippy :: identity_op)] const INACTIVE_SHIFT : usize = 1 * THREADS_BITS ;
    };
}

INACTIVE_SHIFT!();