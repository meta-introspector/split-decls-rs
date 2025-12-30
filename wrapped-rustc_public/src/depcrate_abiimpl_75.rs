// Generated macro for impl_75 (impl)
macro_rules! Depcrate_abiimpl_75 {
() => {
// Module: crate::abi
// Provides: {"impl_75"}
// Dependencies: {}
impl WrappingRange { # [doc = " Returns `true` if `size` completely fills the range."] # [inline] pub fn is_full (& self , size : Size) -> Result < bool , Error > { let Some (max_value) = size . unsigned_int_max () else { return Err (error ! ("Expected size <= 128 bits, but found {} instead" , size . bits ())) ; } ; if self . start <= max_value && self . end <= max_value { Ok (self . start == (self . end . wrapping_add (1) & max_value)) } else { Err (error ! ("Range `{self:?}` out of bounds for size `{}` bits." , size . bits ())) } } # [doc = " Returns `true` if `v` is contained in the range."] # [inline (always)] pub fn contains (& self , v : u128) -> bool { if self . wraps_around () { self . start <= v || v <= self . end } else { self . start <= v && v <= self . end } } # [doc = " Returns `true` if the range wraps around."] # [doc = " I.e., the range represents the union of `self.start..=MAX` and `0..=self.end`."] # [doc = " Returns `false` if this is a non-wrapping range, i.e.: `self.start..=self.end`."] # [inline] pub fn wraps_around (& self) -> bool { self . start > self . end } }
};
}
