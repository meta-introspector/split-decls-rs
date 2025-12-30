// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < T : ConditionallySelectable > ConditionallySelectable for CtOption < T > { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { CtOption :: new (T :: conditional_select (& a . value , & b . value , choice) , Choice :: conditional_select (& a . is_some , & b . is_some , choice) ,) } }
};
}
