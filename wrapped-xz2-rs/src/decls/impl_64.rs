macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < W : Read + Write > Read for XzEncoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
    };
}

impl_64!()