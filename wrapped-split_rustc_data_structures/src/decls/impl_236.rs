macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < 'a , T > Interned < 'a , T > { # [doc = " Create a new `Interned` value. The value referred to *must* be interned"] # [doc = " and thus be unique, and it *must* remain unique in the future. This"] # [doc = " function has `_unchecked` in the name but is not `unsafe`, because if"] # [doc = " the uniqueness condition is violated condition it will cause incorrect"] # [doc = " behaviour but will not affect memory safety."] # [inline] pub const fn new_unchecked (t : & 'a T) -> Self { Interned (t , private :: PrivateZst) } }
    };
}

impl_236!();