macro_rules! deps {
    () => {
        Tag!();
        Result!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl fmt :: Debug for Tag { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("Tag") . field (& self . data) . finish () } }
    };
}

impl_25!();