macro_rules! deps {
    () => {
        SplitAt!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        unsafe impl < T > SplitAt for [T] { type Elem = T ; # [inline] # [allow (dead_code)] fn only_derive_is_allowed_to_implement_this_trait () where Self : Sized , { } }
    };
}

impl_399!();