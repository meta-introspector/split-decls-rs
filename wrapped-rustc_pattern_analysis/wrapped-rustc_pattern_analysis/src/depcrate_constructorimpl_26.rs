// Generated macro for impl_26 (impl)
macro_rules! Depcrate_constructorimpl_26 {
() => {
// Module: crate::constructor
// Provides: {"impl_26"}
// Dependencies: {}
# [doc = " Note: this will render signed ranges incorrectly. To render properly, convert to a pattern"] # [doc = " first."] impl fmt :: Debug for IntRange { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_singleton () { let Finite (lo) = self . lo else { unreachable ! () } ; write ! (f , "{lo}") ? ; } else { if let Finite (lo) = self . lo { write ! (f , "{lo}") ? ; } write ! (f , "{}" , RangeEnd :: Excluded) ? ; if let Finite (hi) = self . hi { write ! (f , "{hi}") ? ; } } Ok (()) } }
};
}
