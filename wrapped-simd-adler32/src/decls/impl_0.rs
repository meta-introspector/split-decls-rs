macro_rules! deps {
    () => {
        Adler32Hash!();
        Adler32!();
    };
}

macro_rules! impl_0 {
    () => {
        deps!();
        impl Adler32Hash for & [u8] { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self) ; hash . finish () } }
    };
}

impl_0!()