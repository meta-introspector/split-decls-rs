macro_rules! deps {
    () => {
        Value!();
        ValueSerializer!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl fmt :: Display for Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use serde_core :: Serialize as _ ; let mut output = String :: new () ; let serializer = crate :: ser :: ValueSerializer :: new (& mut output) ; self . serialize (serializer) . unwrap () ; output . fmt (f) } }
    };
}

impl_83!()