macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl DepKind { # [inline] pub const fn new (variant : u16) -> Self { Self { variant } } # [inline] pub const fn as_inner (& self) -> u16 { self . variant } # [inline] pub const fn as_usize (& self) -> usize { self . variant as usize } }
    };
}

impl_14!()