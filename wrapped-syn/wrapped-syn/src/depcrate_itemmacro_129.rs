// Generated macro for macro_129 (macro)
macro_rules! Depcrate_itemmacro_129 {
() => {
// Module: crate::item
// Provides: {"macro_129"}
// Dependencies: {}
ast_enum_of_structs ! { pub enum TraitItemKind { pub Const (TraitItemConst { pub const_token : tokens :: Const , pub ident : Ident , pub colon_token : tokens :: Colon , pub ty : Ty , pub default : Option < (tokens :: Eq , Expr) >, pub semi_token : tokens :: Semi , }) , pub Method (TraitItemMethod { pub sig : MethodSig , pub default : Option < Block >, pub semi_token : Option < tokens :: Semi >, }) , pub Type (TraitItemType { pub type_token : tokens :: Type , pub ident : Ident , pub colon_token : Option < tokens :: Colon >, pub bounds : Delimited < TyParamBound , tokens :: Add >, pub default : Option < (tokens :: Eq , Ty) >, pub semi_token : tokens :: Semi , }) , pub Macro (Mac) , } do_not_generate_to_tokens }
};
}
