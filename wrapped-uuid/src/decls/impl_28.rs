macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for Uuid { # [inline] fn default () -> Self { Uuid :: nil () } }
    };
}

impl_28!()