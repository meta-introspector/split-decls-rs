// Generated macro for impl_129 (impl)
macro_rules! Depcrate_builder_expr_as_constantimpl_129 {
() => {
// Module: crate::builder::expr::as_constant
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a , 'tcx > Builder < 'a , 'tcx > { # [doc = " Compile `expr`, yielding a compile-time constant. Assumes that"] # [doc = " `expr` is a valid compile-time constant!"] pub (crate) fn as_constant (& mut self , expr : & Expr < 'tcx >) -> ConstOperand < 'tcx > { let this = self ; let tcx = this . tcx ; let Expr { ty , temp_lifetime : _ , span , ref kind } = * expr ; match kind { ExprKind :: Scope { region_scope : _ , lint_level : _ , value } => { this . as_constant (& this . thir [* value]) } _ => as_constant_inner (expr , | user_ty | { Some (this . canonical_user_type_annotations . push (CanonicalUserTypeAnnotation { span , user_ty : user_ty . clone () , inferred_ty : ty , })) } , tcx ,) , } } }
};
}
