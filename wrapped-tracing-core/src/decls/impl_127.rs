macro_rules! deps {
    () => {
        Field!();
        Value!();
        Visit!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T : crate :: field :: Value > crate :: field :: Value for Wrapping < T > { fn record (& self , key : & crate :: field :: Field , visitor : & mut dyn crate :: field :: Visit) { self . 0 . record (key , visitor) } }
    };
}

impl_127!();