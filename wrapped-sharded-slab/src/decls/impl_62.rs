macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T > Clear for Vec < T > { # [inline] fn clear (& mut self) { Vec :: clear (self) } }
    };
}

impl_62!();