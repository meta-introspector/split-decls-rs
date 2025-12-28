macro_rules! deps {
    () => {
        InputStruct!();
        SalsaStructAllowedOptions!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl SalsaStructAllowedOptions for InputStruct { const KIND : & 'static str = "input" ; const ALLOW_MAYBE_UPDATE : bool = false ; const ALLOW_TRACKED : bool = false ; const HAS_LIFETIME : bool = false ; const ELIDABLE_LIFETIME : bool = false ; const ALLOW_DEFAULT : bool = true ; }
    };
}

impl_37!();