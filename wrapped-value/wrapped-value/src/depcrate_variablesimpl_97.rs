// Generated macro for impl_97 (impl)
macro_rules! Depcrate_variablesimpl_97 {
() => {
// Module: crate::variables
// Provides: {"impl_97"}
// Dependencies: {}
impl Variables { # [doc = " Get the variables from a GraphQL value."] # [doc = ""] # [doc = " If the value is not a map, then no variables will be returned."] # [must_use] pub fn from_value (value : ConstValue) -> Self { match value { ConstValue :: Object (obj) => Self (obj . into_iter () . collect ()) , _ => Self :: default () , } } # [doc = " Get the values from a JSON value."] # [doc = ""] # [doc = " If the value is not a map or the keys of a map are not valid GraphQL"] # [doc = " names, then no variables will be returned."] # [must_use] pub fn from_json (value : serde_json :: Value) -> Self { ConstValue :: from_json (value) . map (Self :: from_value) . unwrap_or_default () } # [doc = " Get the variables as a GraphQL value."] # [must_use] pub fn into_value (self) -> ConstValue { ConstValue :: Object (self . 0 . into_iter () . collect ()) } }
};
}
