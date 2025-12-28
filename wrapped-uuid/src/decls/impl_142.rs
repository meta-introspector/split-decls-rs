macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl Default for Uuid { # [inline] fn default () -> Self { Uuid :: nil () } }
    };
}

impl_142!()