macro_rules! deps {
    () => {
        SERVICE_TABLE_ENTRYW!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for SERVICE_TABLE_ENTRYW { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_28!();