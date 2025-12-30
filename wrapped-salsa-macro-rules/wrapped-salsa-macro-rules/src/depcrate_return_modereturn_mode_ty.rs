// Generated macro for return_mode_ty (macro)
macro_rules! Depcrate_return_modereturn_mode_ty {
() => {
// Module: crate::return_mode
// Provides: {"return_mode_ty"}
// Dependencies: {}
# [macro_export] macro_rules ! return_mode_ty { ((copy , $ maybe_backdate : ident , $ maybe_default : ident) , $ db_lt : lifetime , $ field_ty : ty) => { $ field_ty } ; ((clone , $ maybe_backdate : ident , $ maybe_default : ident) , $ db_lt : lifetime , $ field_ty : ty) => { $ field_ty } ; ((ref , $ maybe_backdate : ident , $ maybe_default : ident) , $ db_lt : lifetime , $ field_ty : ty) => { & $ db_lt $ field_ty } ; ((deref , $ maybe_backdate : ident , $ maybe_default : ident) , $ db_lt : lifetime , $ field_ty : ty) => { & $ db_lt <$ field_ty as :: core :: ops :: Deref >:: Target } ; ((as_ref , $ maybe_backdate : ident , $ maybe_default : ident) , $ db_lt : lifetime , $ field_ty : ty) => { <$ field_ty as :: salsa :: SalsaAsRef >:: AsRef <$ db_lt > } ; ((as_deref , $ maybe_backdate : ident , $ maybe_default : ident) , $ db_lt : lifetime , $ field_ty : ty) => { <$ field_ty as :: salsa :: SalsaAsDeref >:: AsDeref <$ db_lt > } ; }
};
}
