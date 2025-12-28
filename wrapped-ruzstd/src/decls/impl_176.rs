macro_rules! deps {
    () => {
        FSEScratch!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl Default for FSEScratch { fn default () -> Self { Self :: new () } }
    };
}

impl_176!();