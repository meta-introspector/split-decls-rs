macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl fmt :: Display for SerializerError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { SerializerError (ref s) => fmt . write_str (s) , } } }
    };
}

impl_42!();