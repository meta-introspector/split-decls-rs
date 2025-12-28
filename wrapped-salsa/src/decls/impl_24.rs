macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl fmt :: Debug for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "Backtrace ") ? ; let mut dbg = fmt . debug_list () ; for frame in & self . 0 { dbg . entry (& frame) ; } dbg . finish () } }
    };
}

impl_24!();