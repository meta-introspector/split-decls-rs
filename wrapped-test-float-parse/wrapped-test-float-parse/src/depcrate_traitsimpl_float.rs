// Generated macro for impl_float (macro)
macro_rules! Depcrate_traitsimpl_float {
() => {
// Module: crate::traits
// Provides: {"impl_float"}
// Dependencies: {}
macro_rules ! impl_float { ($ ($ fty : ty , $ ity : ty) ;+) => { $ (impl Float for $ fty { type Int = $ ity ; type SInt = < Self :: Int as Int >:: Signed ; const BITS : u32 = <$ ity >:: BITS ; const MAN_BITS : u32 = Self :: MANTISSA_DIGITS - 1 ; const MAN_MASK : Self :: Int = (Self :: Int :: ONE << Self :: MAN_BITS) - Self :: Int :: ONE ; const SIGN_MASK : Self :: Int = Self :: Int :: ONE << (Self :: BITS - 1) ; fn from_bits (i : Self :: Int) -> Self { Self :: from_bits (i) } fn to_bits (self) -> Self :: Int { self . to_bits () } fn constants () -> &'static Constants { use std :: sync :: LazyLock ; static CONSTANTS : LazyLock < Constants > = LazyLock :: new (Constants :: new ::<$ fty >) ; & CONSTANTS } }) + } }
};
}
