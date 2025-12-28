macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < W : Write > Write for XzDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
    };
}

impl_41!();