// Generated macro for impl_12 (impl)
macro_rules! Depcrate_genericimpl_12 {
() => {
// Module: crate::generic
// Provides: {"impl_12"}
// Dependencies: {}
impl < REG : Writable > Reg < REG > where REG :: Ux : Default , { # [doc = " Writes 0 to a `Writable` register."] # [doc = ""] # [doc = " Similar to `write`, but unused bits will contain 0."] # [inline (always)] pub unsafe fn write_with_zero < F > (& self , f : F) where F : FnOnce (& mut REG :: Writer) -> & mut W < REG > { self . register . set ((* f (& mut REG :: Writer :: from (W { bits : REG :: Ux :: default () , _reg : marker :: PhantomData , }))) . bits ,) ; } }
};
}
