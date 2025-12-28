macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < W : Read + Write > Read for XzDecoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
    };
}

impl_70!();