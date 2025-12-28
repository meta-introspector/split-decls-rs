macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < W : Write + Read > Write for XzEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
    };
}

impl_50!()