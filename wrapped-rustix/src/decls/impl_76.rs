macro_rules! impl_76 {
    () => {
        impl Hash for EventData { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . u64 () . hash (state) } }
    };
}

impl_76!()