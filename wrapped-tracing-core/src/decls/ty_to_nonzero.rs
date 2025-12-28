macro_rules! ty_to_nonzero {
    () => {
        macro_rules ! ty_to_nonzero { (u8) => { NonZeroU8 } ; (u16) => { NonZeroU16 } ; (u32) => { NonZeroU32 } ; (u64) => { NonZeroU64 } ; (u128) => { NonZeroU128 } ; (usize) => { NonZeroUsize } ; (i8) => { NonZeroI8 } ; (i16) => { NonZeroI16 } ; (i32) => { NonZeroI32 } ; (i64) => { NonZeroI64 } ; (i128) => { NonZeroI128 } ; (isize) => { NonZeroIsize } ; }
    };
}

ty_to_nonzero!();