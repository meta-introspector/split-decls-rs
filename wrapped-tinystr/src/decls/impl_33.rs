macro_rules! deps {
    () => {
        TinyAsciiStr!();
        ParseError!();
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < const N : usize > UnvalidatedTinyAsciiStr < N > { # [inline] # [doc = " Converts into a [`TinyAsciiStr`]. Fails if the bytes are not valid ASCII."] pub fn try_into_tinystr (self) -> Result < TinyAsciiStr < N > , ParseError > { TinyAsciiStr :: try_from_raw (self . 0) } # [inline] # [doc = " Unsafely converts into a [`TinyAsciiStr`]."] pub const fn from_utf8_unchecked (bytes : [u8 ; N]) -> Self { Self (bytes) } }
    };
}

impl_33!();