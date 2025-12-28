macro_rules! deps {
    () => {
        Variations!();
        Stderr!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl Default for Stderr { fn default () -> Self { Stderr { success : true , stderr : Variations :: default () , } } }
    };
}

impl_172!();