macro_rules! deps {
    () => {
        ExtraFieldMagic!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [allow (dead_code)] impl ExtraFieldMagic { pub const fn literal (x : u16) -> Self { Self (x) } # [inline (always)] pub const fn from_le_bytes (bytes : [u8 ; 2]) -> Self { Self (u16 :: from_le_bytes (bytes)) } # [inline (always)] pub const fn to_le_bytes (self) -> [u8 ; 2] { self . 0 . to_le_bytes () } # [allow (clippy :: wrong_self_convention)] # [inline (always)] pub fn from_le (self) -> Self { Self (u16 :: from_le (self . 0)) } # [allow (clippy :: wrong_self_convention)] # [inline (always)] pub fn to_le (self) -> Self { Self (u16 :: to_le (self . 0)) } pub const ZIP64_EXTRA_FIELD_TAG : Self = Self :: literal (0x0001) ; }
    };
}

impl_151!()