// Generated macro for impl_28 (impl)
macro_rules! Depcrate_biteqimpl_28 {
() => {
// Module: crate::biteq
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : BitEq > core :: fmt :: Debug for BitEqEitherWrapper < '_ , T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . 0 . biteq (self . 1) { self . 0 . fmt (f) } else { self . 0 . fmt (f) ? ; write ! (f , " or ") ? ; self . 1 . fmt (f) } } }
};
}
