macro_rules! MLL_CHAR {
    () => {
        # [doc = " `mll-char = %x09 / %x20-26 / %x28-7E / non-ascii`"] const MLL_CHAR : (u8 , RangeInclusive < u8 > , RangeInclusive < u8 > , RangeInclusive < u8 > ,) = (0x9 , 0x20 ..= 0x26 , 0x28 ..= 0x7E , NON_ASCII) ;
    };
}

MLL_CHAR!();