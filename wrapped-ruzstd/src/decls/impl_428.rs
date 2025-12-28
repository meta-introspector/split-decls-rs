macro_rules! deps {
    () => {
        Read!();
        Error!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < T > Read for & mut T where T : Read , { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > { (* self) . read (buf) } }
    };
}

impl_428!();