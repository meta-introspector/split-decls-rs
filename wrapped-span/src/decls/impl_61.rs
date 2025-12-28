macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (not (feature = "salsa"))] impl SyntaxContext { const MAX_ID : u32 = SALSA_MAX_ID_MIRROR - 1 ; pub const fn into_u32 (self) -> u32 { self . 0 } # [doc = " # Safety"] # [doc = ""] # [doc = " None. This is always safe to call without the `salsa` feature."] pub const unsafe fn from_u32 (u32 : u32) -> Self { Self (u32) } }
    };
}

impl_61!()