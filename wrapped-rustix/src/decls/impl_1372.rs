macro_rules! deps {
    () => {
        CpuSet!();
    };
}

macro_rules! impl_1372 {
    () => {
        deps!();
        impl Default for CpuSet { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_1372!();