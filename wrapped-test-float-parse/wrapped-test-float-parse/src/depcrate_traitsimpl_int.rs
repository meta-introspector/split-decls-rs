// Generated macro for impl_int (macro)
macro_rules! Depcrate_traitsimpl_int {
() => {
// Module: crate::traits
// Provides: {"impl_int"}
// Dependencies: {}
macro_rules ! impl_int { ($ ($ uty : ty , $ sty : ty) ;+) => { $ (impl Int for $ uty { type Signed = $ sty ; type Bytes = [u8 ; Self :: BITS as usize / 8] ; const BITS : u32 = Self :: BITS ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MAX : Self = Self :: MAX ; fn to_signed (self) -> Self :: Signed { self . try_into () . unwrap () } fn wrapping_neg (self) -> Self { self . wrapping_neg () } fn trailing_zeros (self) -> u32 { self . trailing_zeros () } } impl Int for $ sty { type Signed = Self ; type Bytes = [u8 ; Self :: BITS as usize / 8] ; const BITS : u32 = Self :: BITS ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MAX : Self = Self :: MAX ; fn to_signed (self) -> Self :: Signed { self } fn wrapping_neg (self) -> Self { self . wrapping_neg () } fn trailing_zeros (self) -> u32 { self . trailing_zeros () } }) + } }
};
}
