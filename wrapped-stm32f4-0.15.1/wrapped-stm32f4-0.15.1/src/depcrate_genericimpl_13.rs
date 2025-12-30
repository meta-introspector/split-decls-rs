// Generated macro for impl_13 (impl)
macro_rules! Depcrate_genericimpl_13 {
() => {
// Module: crate::generic
// Provides: {"impl_13"}
// Dependencies: {}
impl < REG : Readable + Writable > Reg < REG > { # [doc = " Modifies the contents of the register by reading and then writing it."] # [doc = ""] # [doc = " E.g. to do a read-modify-write sequence to change parts of a register:"] # [doc = " ```ignore"] # [doc = " periph.reg.modify(|r, w| unsafe { w.bits("] # [doc = "    r.bits() | 3"] # [doc = " ) });"] # [doc = " ```"] # [doc = " or"] # [doc = " ```ignore"] # [doc = " periph.reg.modify(|_, w| w"] # [doc = "     .field1().bits(newfield1bits)"] # [doc = "     .field2().set_bit()"] # [doc = "     .field3().variant(VARIANT)"] # [doc = " );"] # [doc = " ```"] # [doc = " Other fields will have the value they had before the call to `modify`."] # [inline (always)] pub fn modify < F > (& self , f : F) where for < 'w > F : FnOnce (& REG :: Reader , & 'w mut REG :: Writer) -> & 'w mut W < REG > { let bits = self . register . get () ; self . register . set (f (& REG :: Reader :: from (R { bits , _reg : marker :: PhantomData , }) , & mut REG :: Writer :: from (W { bits , _reg : marker :: PhantomData , }) ,) . bits ,) ; } }
};
}
