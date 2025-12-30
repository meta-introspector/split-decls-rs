// Generated macro for UnstableDoAsFormatter (trait)
macro_rules! DepcrateUnstableDoAsFormatter {
() => {
// Module: crate
// Provides: {"UnstableDoAsFormatter"}
// Dependencies: {}
# [doc (hidden)] pub trait UnstableDoAsFormatter { type Writer : uWrite + ? Sized ; fn do_as_formatter (& mut self , f : impl FnOnce (& mut Formatter < '_ , Self :: Writer >) -> Result < () , < Self :: Writer as uWrite > :: Error > ,) -> Result < () , < Self :: Writer as uWrite > :: Error > ; }
};
}
