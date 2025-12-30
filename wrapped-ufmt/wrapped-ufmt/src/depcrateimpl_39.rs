// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < W > UnstableDoAsFormatter for W where W : uWrite + ? Sized , { type Writer = W ; fn do_as_formatter (& mut self , f : impl FnOnce (& mut Formatter < '_ , W >) -> Result < () , W :: Error > ,) -> Result < () , W :: Error > { f (& mut Formatter :: new (self)) } }
};
}
