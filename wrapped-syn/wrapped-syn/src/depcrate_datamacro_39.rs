// Generated macro for macro_39 (macro)
macro_rules! Depcrate_datamacro_39 {
() => {
// Module: crate::data
// Provides: {"macro_39"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " Visibility level of an item."] pub enum Visibility { # [doc = " Public, i.e. `pub`."] pub Public (VisPublic { pub pub_token : tokens :: Pub , }) , # [doc = " Crate-visible, i.e. `pub(crate)`."] pub Crate (VisCrate { pub pub_token : tokens :: Pub , pub paren_token : tokens :: Paren , pub crate_token : tokens :: Crate , }) , # [doc = " Restricted, e.g. `pub(self)` or `pub(super)` or `pub(in some::module)`."] pub Restricted (VisRestricted { pub pub_token : tokens :: Pub , pub paren_token : tokens :: Paren , pub in_token : Option < tokens :: In >, pub path : Box < Path >, }) , # [doc = " Inherited, i.e. private."] pub Inherited (VisInherited { }) , } }
};
}
