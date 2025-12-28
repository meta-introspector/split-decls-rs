macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { if self . messages . len () == 1 { formatter . debug_tuple ("Error") . field (& self . messages [0]) . finish () } else { formatter . debug_tuple ("Error") . field (& self . messages) . finish () } } }
    };
}

impl_180!();