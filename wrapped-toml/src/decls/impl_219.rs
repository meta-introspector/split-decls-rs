macro_rules! deps {
    () => {
        DeFloat!();
        DeString!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl Default for DeFloat < '_ > { fn default () -> Self { Self { inner : DeString :: Borrowed ("0.0") , } } }
    };
}

impl_219!()