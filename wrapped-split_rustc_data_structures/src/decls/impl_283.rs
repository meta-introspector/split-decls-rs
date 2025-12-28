macro_rules! deps {
    () => {
        FromDyn!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < T > FromDyn < T > { # [inline (always)] pub fn from (val : T) -> Self { assert ! (crate :: sync :: is_dyn_thread_safe ()) ; FromDyn (val) } # [inline (always)] pub fn derive < O > (& self , val : O) -> FromDyn < O > { FromDyn (val) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 } }
    };
}

impl_283!();