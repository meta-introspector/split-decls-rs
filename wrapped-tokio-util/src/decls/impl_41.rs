macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Default for CancellationToken { fn default () -> CancellationToken { CancellationToken :: new () } }
    };
}

impl_41!();