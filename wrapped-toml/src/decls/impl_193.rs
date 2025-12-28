macro_rules! deps {
    () => {
        DeArray!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl Default for DeArray < 'static > { # [inline] fn default () -> Self { Self { items : Default :: default () , array_of_tables : false , } } }
    };
}

impl_193!();