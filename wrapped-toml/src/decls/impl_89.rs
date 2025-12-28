macro_rules! deps {
    () => {
        Value!();
        SeqDeserializer!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl SeqDeserializer { fn new (vec : Vec < Value >) -> Self { Self { iter : vec . into_iter () , } } }
    };
}

impl_89!();