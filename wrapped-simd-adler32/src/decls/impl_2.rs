macro_rules! deps {
    () => {
        Adler32!();
        Adler32Hash!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        # [cfg (feature = "const-generics")] impl < const SIZE : usize > Adler32Hash for [u8 ; SIZE] { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self) ; hash . finish () } }
    };
}

impl_2!();