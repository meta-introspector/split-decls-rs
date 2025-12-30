// Generated macro for prop_assert_biteq (macro)
macro_rules! Depcrate_biteqprop_assert_biteq {
() => {
// Module: crate::biteq
// Provides: {"prop_assert_biteq"}
// Dependencies: {}
# [macro_export] macro_rules ! prop_assert_biteq { { $ a : expr , $ b : expr $ (,) ? } => { { use $ crate :: biteq :: BitEqWrapper ; let a = $ a ; let b = $ b ; proptest :: prop_assert_eq ! (BitEqWrapper (& a) , BitEqWrapper (& b)) ; } } ; { $ a : expr , $ b : expr , $ c : expr $ (,) ? } => { { use $ crate :: biteq :: { BitEqWrapper , BitEqEitherWrapper } ; let a = $ a ; let b = $ b ; let c = $ c ; proptest :: prop_assert_eq ! (BitEqWrapper (& a) , BitEqEitherWrapper (& b , & c)) ; } } ; }
};
}
