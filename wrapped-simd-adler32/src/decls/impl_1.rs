macro_rules! deps {
    () => {
        Adler32!();
        Adler32Hash!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Adler32Hash for & str { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self . as_bytes ()) ; hash . finish () } }
    };
}

impl_1!()