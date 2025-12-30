// Generated macro for uDebug (trait)
macro_rules! DepcrateuDebug {
() => {
// Module: crate
// Provides: {"uDebug"}
// Dependencies: {}
# [doc = " Just like `core::fmt::Debug`"] # [allow (non_camel_case_types)] pub trait uDebug { # [doc = " Formats the value using the given formatter"] fn fmt < W > (& self , _ : & mut Formatter < '_ , W >) -> Result < () , W :: Error > where W : uWrite + ? Sized ; }
};
}
