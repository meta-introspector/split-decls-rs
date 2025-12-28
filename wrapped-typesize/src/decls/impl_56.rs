macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl TypeSize for url :: Url { fn extra_size (& self) -> usize { let serialization : String = self . clone () . into () ; serialization . extra_size () } }
    };
}

impl_56!();