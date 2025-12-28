macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl Hash for Id { fn hash < H : Hasher > (& self , state : & mut H) { state . write_u64 (self . as_bits ()) ; } }
    };
}

impl_134!();