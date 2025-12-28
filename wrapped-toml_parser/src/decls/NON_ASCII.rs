macro_rules! NON_ASCII {
    () => {
        pub (crate) const NON_ASCII : RangeInclusive < u8 > = 0x80 ..= 0xff ;
    };
}

NON_ASCII!();