macro_rules! deps {
    () => {
        ParseError!();
        TinyAsciiStr!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < const N : usize > FromStr for TinyAsciiStr < N > { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
    };
}

impl_13!()