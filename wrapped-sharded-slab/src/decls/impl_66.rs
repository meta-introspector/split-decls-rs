macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T : Clear > Clear for sync :: Mutex < T > { # [inline] fn clear (& mut self) { self . get_mut () . unwrap () . clear () ; } }
    };
}

impl_66!();