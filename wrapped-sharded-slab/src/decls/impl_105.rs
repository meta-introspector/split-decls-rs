macro_rules! deps {
    () => {
        RefCount!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < C > fmt :: Debug for RefCount < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("RefCount") . field (& self . value) . finish () } }
    };
}

impl_105!()