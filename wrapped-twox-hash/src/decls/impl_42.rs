macro_rules! deps {
    () => {
        BufferData!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl fmt :: Debug for BufferData { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . 0 . iter ()) . finish () } }
    };
}

impl_42!();