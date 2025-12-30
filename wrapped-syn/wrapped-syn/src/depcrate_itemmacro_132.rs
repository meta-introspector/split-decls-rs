// Generated macro for macro_132 (macro)
macro_rules! Depcrate_itemmacro_132 {
() => {
// Module: crate::item
// Provides: {"macro_132"}
// Dependencies: {}
ast_enum_of_structs ! { pub enum ImplItemKind { pub Const (ImplItemConst { pub vis : Visibility , pub defaultness : Defaultness , pub const_token : tokens :: Const , pub ident : Ident , pub colon_token : tokens :: Colon , pub ty : Ty , pub eq_token : tokens :: Eq , pub expr : Expr , pub semi_token : tokens :: Semi , }) , pub Method (ImplItemMethod { pub vis : Visibility , pub defaultness : Defaultness , pub sig : MethodSig , pub block : Block , }) , pub Type (ImplItemType { pub vis : Visibility , pub defaultness : Defaultness , pub type_token : tokens :: Type , pub ident : Ident , pub eq_token : tokens :: Eq , pub ty : Ty , pub semi_token : tokens :: Semi , }) , pub Macro (Mac) , } do_not_generate_to_tokens }
};
}
