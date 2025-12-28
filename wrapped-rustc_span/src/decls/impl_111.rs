macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl fmt :: Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* DEF_ID_DEBUG) (* self , f) } }
    };
}

impl_111!()