macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for Builder < '_ , '_ > { fn default () -> Self { Builder { random_len : crate :: NUM_RAND_CHARS , prefix : OsStr :: new (".tmp") , suffix : OsStr :: new ("") , append : false , permissions : None , disable_cleanup : false , } } }
    };
}

impl_10!()