macro_rules! deps {
    () => {
        Error!();
        Write!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl < T > Write for & mut T where T : Write , { fn write (& mut self , buf : & [u8]) -> Result < usize , Error > { (* self) . write (buf) } fn flush (& mut self) -> Result < () , Error > { (* self) . flush () } }
    };
}

impl_433!();