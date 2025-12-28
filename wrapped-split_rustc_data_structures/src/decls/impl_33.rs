macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Hash for Fingerprint { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { state . write_fingerprint (self) ; } }
    };
}

impl_33!()