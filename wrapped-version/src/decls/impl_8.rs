macro_rules! deps {
    () => {
        OSVERSIONINFOW!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Default for OSVERSIONINFOW { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_8!();