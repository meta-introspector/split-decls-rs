macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Default for Context { fn default () -> Context { Context :: new () } }
    };
}

impl_99!();