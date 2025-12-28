macro_rules! deps {
    () => {
        AsciiWordBoundIter!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a > AsciiWordBoundIter < 'a > { pub fn new (s : & 'a str) -> Self { AsciiWordBoundIter { rest : s , offset : 0 } } # [inline] fn is_core (b : u8) -> bool { b . is_ascii_alphanumeric () || b == b'_' } # [inline] fn is_infix (b : u8 , prev : u8 , next : u8) -> bool { match b { b'.' | b',' | b';' | b'\'' if prev . is_ascii_digit () && next . is_ascii_digit () => true , b'\'' | b'.' | b':' if prev . is_ascii_alphabetic () && next . is_ascii_alphabetic () => true , _ => false , } } }
    };
}

impl_63!();