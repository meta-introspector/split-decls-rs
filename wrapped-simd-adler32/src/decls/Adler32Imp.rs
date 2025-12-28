macro_rules! Adler32Imp {
    () => {
        pub type Adler32Imp = fn (u16 , u16 , & [u8]) -> (u16 , u16) ;
    };
}

Adler32Imp!();