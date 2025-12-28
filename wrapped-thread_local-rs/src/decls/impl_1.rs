macro_rules! deps {
    () => {
        CachedThreadLocal!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < T : Send > Default for CachedThreadLocal < T > { fn default () -> CachedThreadLocal < T > { CachedThreadLocal :: new () } }
    };
}

impl_1!()