macro_rules! deps {
    () => {
        BoundConstness!();
        Const!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl fmt :: Display for BoundConstness { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Const => f . write_str ("const") , Self :: Maybe => f . write_str ("[const]") , } } }
    };
}

impl_400!()