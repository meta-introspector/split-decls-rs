// Generated macro for macro_122 (macro)
macro_rules! Depcrate_itemmacro_122 {
() => {
// Module: crate::item
// Provides: {"macro_122"}
// Dependencies: {}
ast_enum_of_structs ! { pub enum ViewPath { # [doc = " `foo::bar::baz as quux`"] # [doc = ""] # [doc = " or just"] # [doc = ""] # [doc = " `foo::bar::baz` (with `as baz` implicitly on the right)"] pub Simple (PathSimple { pub path : Path , pub as_token : Option < tokens :: As >, pub rename : Option < Ident >, }) , # [doc = " `foo::bar::*`"] pub Glob (PathGlob { pub path : Path , pub colon2_token : Option < tokens :: Colon2 >, pub star_token : tokens :: Star , }) , # [doc = " `foo::bar::{a, b, c}`"] pub List (PathList { pub path : Path , pub colon2_token : tokens :: Colon2 , pub brace_token : tokens :: Brace , pub items : Delimited < PathListItem , tokens :: Comma >, }) , } }
};
}
