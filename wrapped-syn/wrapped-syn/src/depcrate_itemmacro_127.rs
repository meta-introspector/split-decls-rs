// Generated macro for macro_127 (macro)
macro_rules! Depcrate_itemmacro_127 {
() => {
// Module: crate::item
// Provides: {"macro_127"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " An item within an `extern` block"] pub enum ForeignItemKind { # [doc = " A foreign function"] pub Fn (ForeignItemFn { pub decl : Box < FnDecl >, }) , # [doc = " A foreign static item (`static ext: u8`)"] pub Static (ForeignItemStatic { pub static_token : tokens :: Static , pub ty : Box < Ty >, pub colon_token : tokens :: Colon , pub mutbl : Mutability , }) , } do_not_generate_to_tokens }
};
}
