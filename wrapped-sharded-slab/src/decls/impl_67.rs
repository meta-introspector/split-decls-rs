macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T : Clear > Clear for sync :: RwLock < T > { # [inline] fn clear (& mut self) { self . write () . unwrap () . clear () ; } }
    };
}

impl_67!();