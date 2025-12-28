macro_rules! BASIC_UNESCAPED {
    () => {
        # [doc = " `basic-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii`"] # [allow (clippy :: type_complexity)] const BASIC_UNESCAPED : ((u8 , u8) , u8 , RangeInclusive < u8 > , RangeInclusive < u8 > , RangeInclusive < u8 > ,) = (WSCHAR , 0x21 , 0x23 ..= 0x5B , 0x5D ..= 0x7E , NON_ASCII) ;
    };
}

BASIC_UNESCAPED!()