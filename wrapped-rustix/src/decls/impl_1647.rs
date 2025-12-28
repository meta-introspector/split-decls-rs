macro_rules! deps {
    () => {
        KernelSigSet!();
    };
}

macro_rules! impl_1647 {
    () => {
        deps!();
        impl Default for KernelSigSet { # [inline] fn default () -> Self { Self :: empty () } }
    };
}

impl_1647!()