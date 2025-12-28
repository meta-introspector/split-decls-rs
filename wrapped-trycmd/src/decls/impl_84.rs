macro_rules! deps {
    () => {
        RunnerSpec!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Default for RunnerSpec { fn default () -> Self { Self :: new () } }
    };
}

impl_84!();