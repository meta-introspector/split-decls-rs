macro_rules! deps {
    () => {
        SalsaStructAllowedOptions!();
        InternedStruct!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl SalsaStructAllowedOptions for InternedStruct { const KIND : & 'static str = "interned" ; const ALLOW_MAYBE_UPDATE : bool = false ; const ALLOW_TRACKED : bool = false ; const HAS_LIFETIME : bool = true ; const ELIDABLE_LIFETIME : bool = true ; const ALLOW_DEFAULT : bool = false ; }
    };
}

impl_45!();