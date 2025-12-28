macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl core :: fmt :: Display for SerializerError { fn fmt (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: InvalidFormat (e) => e . fmt (formatter) , Self :: InvalidProtocol => "invalid serialization protocol" . fmt (formatter) , } } }
    };
}

impl_55!();