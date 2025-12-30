// Generated macro for return_mode_expression (macro)
macro_rules! Depcrate_return_modereturn_mode_expression {
() => {
// Module: crate::return_mode
// Provides: {"return_mode_expression"}
// Dependencies: {}
# [doc = " Generate the expression for the return type, depending on the return mode defined in [`salsa-macros::options::Options::returns`]"] # [doc = ""] # [doc = " Used when generating field getters."] # [macro_export] macro_rules ! return_mode_expression { ((copy , $ maybe_backdate : ident , $ maybe_default : ident) , $ field_ty : ty , $ field_ref_expr : expr ,) => { *$ field_ref_expr } ; ((clone , $ maybe_backdate : ident , $ maybe_default : ident) , $ field_ty : ty , $ field_ref_expr : expr ,) => { :: core :: clone :: Clone :: clone ($ field_ref_expr) } ; ((ref , $ maybe_backdate : ident , $ maybe_default : ident) , $ field_ty : ty , $ field_ref_expr : expr ,) => { $ field_ref_expr } ; ((deref , $ maybe_backdate : ident , $ maybe_default : ident) , $ field_ty : ty , $ field_ref_expr : expr ,) => { :: core :: ops :: Deref :: deref ($ field_ref_expr) } ; ((as_ref , $ maybe_backdate : ident , $ maybe_default : ident) , $ field_ty : ty , $ field_ref_expr : expr ,) => { :: salsa :: SalsaAsRef :: as_ref ($ field_ref_expr) } ; ((as_deref , $ maybe_backdate : ident , $ maybe_default : ident) , $ field_ty : ty , $ field_ref_expr : expr ,) => { :: salsa :: SalsaAsDeref :: as_deref ($ field_ref_expr) } ; }
};
}
