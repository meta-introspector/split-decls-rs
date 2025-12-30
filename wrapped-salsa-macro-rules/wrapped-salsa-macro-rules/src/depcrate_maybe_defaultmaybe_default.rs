// Generated macro for maybe_default (macro)
macro_rules! Depcrate_maybe_defaultmaybe_default {
() => {
// Module: crate::maybe_default
// Provides: {"maybe_default"}
// Dependencies: {}
# [doc = " Generate either `field_ref_expr` or `field_ty::default`"] # [doc = ""] # [doc = " Used when generating an input's builder."] # [macro_export] macro_rules ! maybe_default { (($ return_mode : ident , $ maybe_backdate : ident , default) , $ field_ty : ty , $ field_ref_expr : expr ,) => { <$ field_ty >:: default () } ; (($ return_mode : ident , $ maybe_backdate : ident , required) , $ field_ty : ty , $ field_ref_expr : expr ,) => { $ field_ref_expr } ; }
};
}
