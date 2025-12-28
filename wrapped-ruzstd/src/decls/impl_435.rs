macro_rules! deps {
    () => {
        Write!();
        Error!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl Write for alloc :: vec :: Vec < u8 > { # [inline] fn write (& mut self , data : & [u8]) -> Result < usize , Error > { self . extend_from_slice (data) ; Ok (data . len ()) } fn flush (& mut self) -> Result < () , Error > { Ok (()) } }
    };
}

impl_435!()