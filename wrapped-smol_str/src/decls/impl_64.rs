macro_rules! deps {
    () => {
        SmolStrBuilder!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl fmt :: Write for SmolStrBuilder { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_str (s) ; Ok (()) } }
    };
}

impl_64!();