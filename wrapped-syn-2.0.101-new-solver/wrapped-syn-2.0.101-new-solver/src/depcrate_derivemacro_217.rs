// Generated macro for macro_217 (macro)
macro_rules! Depcrate_derivemacro_217 {
() => {
// Module: crate::derive
// Provides: {"macro_217"}
// Dependencies: {}
ast_enum ! { # [doc = " The storage of a struct, enum or union data structure."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub enum Data { Struct (DataStruct) , Enum (DataEnum) , Union (DataUnion) , } }
};
}
