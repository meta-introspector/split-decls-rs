macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < R : Read > Read for XzDecoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
    };
}

impl_53!();