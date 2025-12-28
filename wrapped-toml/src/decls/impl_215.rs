macro_rules! deps {
    () => {
        DeString!();
        DeInteger!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl Default for DeInteger < '_ > { fn default () -> Self { Self { inner : DeString :: Borrowed ("0") , radix : 10 , } } }
    };
}

impl_215!();