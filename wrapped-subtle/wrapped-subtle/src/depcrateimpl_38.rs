// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl ConditionallySelectable for Choice { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Choice (u8 :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
