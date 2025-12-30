// Generated macro for macro_69 (macro)
macro_rules! Depcrate_genericsmacro_69 {
() => {
// Module: crate::generics
// Provides: {"macro_69"}
// Dependencies: {}
ast_struct ! { # [doc = " Represents lifetimes and type parameters attached to a declaration"] # [doc = " of a function, enum, trait, etc."] # [derive (Default)] pub struct Generics { pub lt_token : Option < tokens :: Lt >, pub gt_token : Option < tokens :: Gt >, pub lifetimes : Delimited < LifetimeDef , tokens :: Comma >, pub ty_params : Delimited < TyParam , tokens :: Comma >, pub where_clause : WhereClause , } }
};
}
