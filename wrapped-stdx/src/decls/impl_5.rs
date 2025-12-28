macro_rules! deps {
    () => {
        TypeIdHasher!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Hasher for TypeIdHasher { # [inline] fn write (& mut self , bytes : & [u8]) { debug_assert_eq ! (bytes . len () , 8) ; let _ = bytes . try_into () . map (| array | self . value = u64 :: from_ne_bytes (array)) ; } # [inline] fn finish (& self) -> u64 { self . value } }
    };
}

impl_5!()