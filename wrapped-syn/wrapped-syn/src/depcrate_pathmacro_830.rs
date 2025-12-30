// Generated macro for macro_830 (macro)
macro_rules! Depcrate_pathmacro_830 {
() => {
// Module: crate::path
// Provides: {"macro_830"}
// Dependencies: {}
ast_enum ! { # [doc = " Angle bracketed or parenthesized arguments of a path segment."] # [doc = ""] # [doc = " ## Angle bracketed"] # [doc = ""] # [doc = " The `<'a, T>` in `std::slice::iter<'a, T>`."] # [doc = ""] # [doc = " ## Parenthesized"] # [doc = ""] # [doc = " The `(A, B) -> C` in `Fn(A, B) -> C`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum PathArguments { None , # [doc = " The `<'a, T>` in `std::slice::iter<'a, T>`."] AngleBracketed (AngleBracketedGenericArguments) , # [doc = " The `(A, B) -> C` in `Fn(A, B) -> C`."] Parenthesized (ParenthesizedGenericArguments) , } }
};
}
