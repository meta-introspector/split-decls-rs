macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < R : Read > Read for XzEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
    };
}

impl_48!();