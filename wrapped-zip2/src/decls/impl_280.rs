macro_rules! deps {
    () => {
        ZipCryptoWriter!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < W : std :: io :: Write > std :: io :: Write for ZipCryptoWriter < W > { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . buffer . extend_from_slice (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
    };
}

impl_280!();