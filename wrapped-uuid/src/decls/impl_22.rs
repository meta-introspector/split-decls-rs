macro_rules! deps {
    () => {
        NonNilUuid!();
        Uuid!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl fmt :: Debug for NonNilUuid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& Uuid :: from (* self) , f) } }
    };
}

impl_22!()