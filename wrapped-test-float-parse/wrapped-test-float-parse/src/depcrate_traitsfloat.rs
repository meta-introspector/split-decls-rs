// Generated macro for Float (trait)
macro_rules! Depcrate_traitsFloat {
() => {
// Module: crate::traits
// Provides: {"Float"}
// Dependencies: {}
# [doc = " Floating point types."] pub trait Float : Copy + fmt :: Debug + fmt :: LowerExp + FromStr < Err : fmt :: Display > + Sized + Send + 'static { # [doc = " Unsigned integer of same width"] type Int : Int < Signed = Self :: SInt > ; type SInt : Int ; # [doc = " Total bits"] const BITS : u32 ; # [doc = " (Stored) bits in the mantissa)"] const MAN_BITS : u32 ; # [doc = " Bits in the exponent"] const EXP_BITS : u32 = Self :: BITS - Self :: MAN_BITS - 1 ; # [doc = " A saturated exponent (all ones)"] const EXP_SAT : u32 = (1 << Self :: EXP_BITS) - 1 ; # [doc = " The exponent bias, also its maximum value"] const EXP_BIAS : u32 = Self :: EXP_SAT >> 1 ; const MAN_MASK : Self :: Int ; const SIGN_MASK : Self :: Int ; fn from_bits (i : Self :: Int) -> Self ; fn to_bits (self) -> Self :: Int ; # [doc = " Rational constants associated with this float type."] fn constants () -> & 'static Constants ; fn is_sign_negative (self) -> bool { (self . to_bits () & Self :: SIGN_MASK) > Self :: Int :: ZERO } # [doc = " Exponent without adjustment for bias."] fn exponent (self) -> u32 { ((self . to_bits () >> Self :: MAN_BITS) & Self :: EXP_SAT . try_into () . unwrap ()) . try_into () . unwrap () } fn mantissa (self) -> Self :: Int { self . to_bits () & Self :: MAN_MASK } }
};
}
