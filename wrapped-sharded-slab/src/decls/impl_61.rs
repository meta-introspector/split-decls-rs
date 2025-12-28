macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T > Clear for Box < T > where T : Clear , { # [inline] fn clear (& mut self) { self . deref_mut () . clear () } }
    };
}

impl_61!()