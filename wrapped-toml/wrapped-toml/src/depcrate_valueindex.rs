// Generated macro for Index (trait)
macro_rules! Depcrate_valueIndex {
() => {
// Module: crate::value
// Provides: {"Index"}
// Dependencies: {}
# [doc = " Types that can be used to index a `toml::Value`"] # [doc = ""] # [doc = " Currently this is implemented for `usize` to index arrays and `str` to index"] # [doc = " tables."] # [doc = ""] # [doc = " This trait is sealed and not intended for implementation outside of the"] # [doc = " `toml` crate."] pub trait Index : Sealed { # [doc (hidden)] fn index < 'a > (& self , val : & 'a Value) -> Option < & 'a Value > ; # [doc (hidden)] fn index_mut < 'a > (& self , val : & 'a mut Value) -> Option < & 'a mut Value > ; }
};
}
