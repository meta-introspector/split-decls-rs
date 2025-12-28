macro_rules! HEXDIG {
    () => {
        # [doc = " `HEXDIG = DIGIT / \"A\" / \"B\" / \"C\" / \"D\" / \"E\" / \"F\"`"] const HEXDIG : (RangeInclusive < u8 > , RangeInclusive < u8 > , RangeInclusive < u8 >) = (DIGIT , b'A' ..= b'F' , b'a' ..= b'f') ;
    };
}

HEXDIG!()