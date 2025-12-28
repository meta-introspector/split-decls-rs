macro_rules! deps {
    () => {
        Unexpected!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl Default for Unexpected { fn default () -> Self { Unexpected :: None } }
    };
}

impl_511!();