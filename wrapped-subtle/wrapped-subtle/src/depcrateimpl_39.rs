// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (feature = "const-generics")] impl < T , const N : usize > ConditionallySelectable for [T ; N] where T : ConditionallySelectable , { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { let mut output = * a ; output . conditional_assign (b , choice) ; output } fn conditional_assign (& mut self , other : & Self , choice : Choice) { for (a_i , b_i) in self . iter_mut () . zip (other) { a_i . conditional_assign (b_i , choice) } } }
};
}
