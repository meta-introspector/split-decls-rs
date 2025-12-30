// Generated macro for impl_11 (impl)
macro_rules! Depcrate_arrayimpl_11 {
() => {
// Module: crate::array
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " Constructors"] # [doc = ""] # [doc = " See also `FromIterator`"] impl Array { # [doc = " Create an empty `Array`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut arr = toml_edit::Array::new();"] # [doc = " ```"] pub fn new () -> Self { Default :: default () } pub (crate) fn with_vec (values : Vec < Item >) -> Self { Self { values , .. Default :: default () } } }
};
}
