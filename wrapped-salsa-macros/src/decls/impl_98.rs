macro_rules! deps {
    () => {
        SalsaStructAllowedOptions!();
        TrackedStruct!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl SalsaStructAllowedOptions for TrackedStruct { const KIND : & 'static str = "tracked" ; const ALLOW_MAYBE_UPDATE : bool = true ; const ALLOW_TRACKED : bool = true ; const HAS_LIFETIME : bool = true ; const ELIDABLE_LIFETIME : bool = false ; const ALLOW_DEFAULT : bool = false ; }
    };
}

impl_98!()