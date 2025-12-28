macro_rules! UNQUOTED_CHAR {
    () => {
        # [doc = " `unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _`"] const UNQUOTED_CHAR : (RangeInclusive < u8 > , RangeInclusive < u8 > , RangeInclusive < u8 > , u8 , u8 ,) = (b'A' ..= b'Z' , b'a' ..= b'z' , b'0' ..= b'9' , b'-' , b'_') ;
    };
}

UNQUOTED_CHAR!();