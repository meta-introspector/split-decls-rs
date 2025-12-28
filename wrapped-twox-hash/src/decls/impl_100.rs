macro_rules! deps {
    () => {
        RawHasher!();
        Finalize64!();
        Hasher!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < S > hash :: Hasher for RawHasher < S > where S : FixedBuffer , { # [inline] fn write (& mut self , input : & [u8]) { self . 0 . write (input) ; } # [inline] fn finish (& self) -> u64 { self . 0 . finish (Finalize64) } }
    };
}

impl_100!();