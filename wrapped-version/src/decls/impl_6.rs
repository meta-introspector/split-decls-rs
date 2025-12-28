macro_rules! deps {
    () => {
        OSVERSIONINFOEXW!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Default for OSVERSIONINFOEXW { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_6!()