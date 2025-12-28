macro_rules! deps {
    () => {
        Uuid!();
        NonNilUuid!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl PartialEq < Uuid > for NonNilUuid { fn eq (& self , other : & Uuid) -> bool { self . get () == * other } }
    };
}

impl_24!()