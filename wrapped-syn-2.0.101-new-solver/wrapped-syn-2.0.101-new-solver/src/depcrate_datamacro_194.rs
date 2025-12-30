// Generated macro for macro_194 (macro)
macro_rules! Depcrate_datamacro_194 {
() => {
// Module: crate::data
// Provides: {"macro_194"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " Data stored within an enum variant or struct."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum Fields { # [doc = " Named fields of a struct or struct variant such as `Point { x: f64,"] # [doc = " y: f64 }`."] Named (FieldsNamed) , # [doc = " Unnamed fields of a tuple struct or tuple variant such as `Some(T)`."] Unnamed (FieldsUnnamed) , # [doc = " Unit struct or unit variant such as `None`."] Unit , } }
};
}
