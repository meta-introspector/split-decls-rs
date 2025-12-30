// Generated macro for impl_9 (impl)
macro_rules! Depcrate_genericimpl_9 {
() => {
// Module: crate::generic
// Provides: {"impl_9"}
// Dependencies: {}
impl < REG : RegisterSpec > Reg < REG > { # [doc = " Returns the underlying memory address of register."] # [doc = ""] # [doc = " ```ignore"] # [doc = " let reg_ptr = periph.reg.as_ptr();"] # [doc = " ```"] # [inline (always)] pub fn as_ptr (& self) -> * mut REG :: Ux { self . register . as_ptr () } }
};
}
