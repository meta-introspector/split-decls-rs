macro_rules! deps {
    () => {
        TP_CALLBACK_ENVIRON_V3!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Default for TP_CALLBACK_ENVIRON_V3 { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_17!();