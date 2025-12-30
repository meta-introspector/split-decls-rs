// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > ConditionallyNegatable for T where T : ConditionallySelectable , for < 'a > & 'a T : Neg < Output = T > , { # [inline] fn conditional_negate (& mut self , choice : Choice) { let self_neg : T = - (self as & T) ; self . conditional_assign (& self_neg , choice) ; } }
};
}
