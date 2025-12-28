macro_rules! impl_429 {
    () => {
        impl open_how { # [doc = " Create a zero-initialized `Self`."] pub const fn zeroed () -> Self { Self { flags : 0 , mode : 0 , resolve : ResolveFlags :: empty () , } } }
    };
}

impl_429!()