macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for BSTR { fn default () -> Self { Self (core :: ptr :: null_mut ()) } }
    };
}

impl_10!()