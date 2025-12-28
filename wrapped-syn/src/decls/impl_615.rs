macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        impl < T , P > Default for Punctuated < T , P > { fn default () -> Self { Punctuated :: new () } }
    };
}

impl_615!()