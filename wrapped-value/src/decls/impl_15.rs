macro_rules! deps {
    () => {
        ConstValue!();
        SeqDeserializer!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl SeqDeserializer { fn new (vec : Vec < ConstValue >) -> Self { SeqDeserializer { iter : vec . into_iter () , } } }
    };
}

impl_15!();