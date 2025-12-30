// Generated macro for uDisplay (trait)
macro_rules! DepcrateuDisplay {
() => {
// Module: crate
// Provides: {"uDisplay"}
// Dependencies: {}
# [doc = " Just like `core::fmt::Display`"] # [allow (non_camel_case_types)] pub trait uDisplay { # [doc = " Formats the value using the given formatter"] fn fmt < W > (& self , _ : & mut Formatter < '_ , W >) -> Result < () , W :: Error > where W : uWrite + ? Sized ; }
};
}
