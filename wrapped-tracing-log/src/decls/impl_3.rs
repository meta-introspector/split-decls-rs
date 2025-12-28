macro_rules! deps {
    () => {
        LogTracer!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Default for LogTracer { fn default () -> Self { Self :: new () } }
    };
}

impl_3!();