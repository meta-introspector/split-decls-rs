macro_rules! deps {
    () => {
        DirRoot!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl Default for DirRoot { fn default () -> Self { Self :: none () } }
    };
}

impl_195!();