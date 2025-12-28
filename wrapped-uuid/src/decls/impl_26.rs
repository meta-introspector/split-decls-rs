macro_rules! deps {
    () => {
        Uuid!();
        NonNilUuid!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl NonNilUuid { # [doc = " Creates a non-nil UUID if the value is non-nil."] pub const fn new (uuid : Uuid) -> Option < Self > { match NonZeroU128 :: new (uuid . as_u128 ()) { Some (non_nil) => Some (NonNilUuid (non_nil)) , None => None , } } # [doc = " Creates a non-nil without checking whether the value is non-nil. This results in undefined behavior if the value is nil."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The value must not be nil."] pub const unsafe fn new_unchecked (uuid : Uuid) -> Self { NonNilUuid (unsafe { NonZeroU128 :: new_unchecked (uuid . as_u128 ()) }) } # [doc = " Get the underlying [`Uuid`] value."] # [inline] pub const fn get (self) -> Uuid { Uuid :: from_u128 (self . 0 . get ()) } }
    };
}

impl_26!();