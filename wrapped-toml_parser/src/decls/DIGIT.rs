macro_rules! DIGIT {
    () => {
        # [doc = " `DIGIT = %x30-39 ; 0-9`"] const DIGIT : RangeInclusive < u8 > = b'0' ..= b'9' ;
    };
}

DIGIT!()