macro_rules! deps {
    () => {
        DepNode!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl fmt :: Debug for DepNode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* DEP_NODE_DEBUG) (* self , f) } }
    };
}

impl_22!();