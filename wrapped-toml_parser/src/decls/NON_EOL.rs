macro_rules! NON_EOL {
    () => {
        pub (crate) const NON_EOL : (u8 , RangeInclusive < u8 > , RangeInclusive < u8 >) = (0x09 , 0x20 ..= 0x7E , NON_ASCII) ;
    };
}

NON_EOL!()