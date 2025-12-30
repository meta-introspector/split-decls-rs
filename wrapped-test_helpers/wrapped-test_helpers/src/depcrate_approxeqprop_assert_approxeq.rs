// Generated macro for prop_assert_approxeq (macro)
macro_rules! Depcrate_approxeqprop_assert_approxeq {
() => {
// Module: crate::approxeq
// Provides: {"prop_assert_approxeq"}
// Dependencies: {}
# [macro_export] macro_rules ! prop_assert_approxeq { { $ a : expr , $ b : expr , $ ulps : expr $ (,) ? } => { { use $ crate :: approxeq :: ApproxEqWrapper ; let a = $ a ; let b = $ b ; proptest :: prop_assert_eq ! (ApproxEqWrapper (& a , $ ulps) , b) ; } } ; }
};
}
