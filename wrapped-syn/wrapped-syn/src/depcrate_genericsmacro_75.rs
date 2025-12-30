// Generated macro for macro_75 (macro)
macro_rules! Depcrate_genericsmacro_75 {
() => {
// Module: crate::generics
// Provides: {"macro_75"}
// Dependencies: {}
ast_struct ! { # [doc = " A set of bound lifetimes, e.g. `for<'a, 'b, 'c>`"] # [derive (Default)] pub struct BoundLifetimes { pub for_token : tokens :: For , pub lt_token : tokens :: Lt , pub lifetimes : Delimited < LifetimeDef , tokens :: Comma >, pub gt_token : tokens :: Gt , } }
};
}
