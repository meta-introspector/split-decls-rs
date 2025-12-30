// Generated macro for impl_11 (impl)
macro_rules! Depcrate_genericimpl_11 {
() => {
// Module: crate::generic
// Provides: {"impl_11"}
// Dependencies: {}
impl < REG : Resettable + Writable > Reg < REG > { # [doc = " Writes the reset value to `Writable` register."] # [doc = ""] # [doc = " Resets the register to its initial state."] # [inline (always)] pub fn reset (& self) { self . register . set (REG :: reset_value ()) } # [doc = " Writes bits to a `Writable` register."] # [doc = ""] # [doc = " You can write raw bits into a register:"] # [doc = " ```ignore"] # [doc = " periph.reg.write(|w| unsafe { w.bits(rawbits) });"] # [doc = " ```"] # [doc = " or write only the fields you need:"] # [doc = " ```ignore"] # [doc = " periph.reg.write(|w| w"] # [doc = "     .field1().bits(newfield1bits)"] # [doc = "     .field2().set_bit()"] # [doc = "     .field3().variant(VARIANT)"] # [doc = " );"] # [doc = " ```"] # [doc = " In the latter case, other fields will be set to their reset value."] # [inline (always)] pub fn write < F > (& self , f : F) where F : FnOnce (& mut REG :: Writer) -> & mut W < REG > { self . register . set (f (& mut REG :: Writer :: from (W { bits : REG :: reset_value () , _reg : marker :: PhantomData , })) . bits ,) ; } }
};
}
