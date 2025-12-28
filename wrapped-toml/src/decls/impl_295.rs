macro_rules! deps {
    () => {
        SerializationStrategy!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl core :: fmt :: Display for SerializationStrategy { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { "error" . fmt (f) } }
    };
}

impl_295!()