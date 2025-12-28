macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T > Clear for Option < T > { fn clear (& mut self) { let _ = self . take () ; } }
    };
}

impl_60!()