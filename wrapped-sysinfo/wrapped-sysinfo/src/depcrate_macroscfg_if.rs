// Generated macro for cfg_if (macro)
macro_rules! Depcrate_macroscfg_if {
() => {
// Module: crate::macros
// Provides: {"cfg_if"}
// Dependencies: {}
macro_rules ! cfg_if { ($ (if # [cfg ($ i_meta : meta)] { $ ($ i_tokens : tt) * }) else + else { $ ($ e_tokens : tt) * }) => { cfg_if ! { @ __items () ; $ ((($ i_meta) ($ ($ i_tokens) *)) ,) + (() ($ ($ e_tokens) *)) , } } ; ($ (if # [cfg ($ i_meta : meta)] { $ ($ i_tokens : tt) * }) else + else { $ ($ e_tokens : tt) * } if $ ($ extra_conditions : tt) +) => { cfg_if ! { @ __items () ; $ ((($ i_meta) ($ ($ i_tokens) *)) ,) + (() ($ ($ e_tokens) *)) , } cfg_if ! { if $ ($ extra_conditions) + } } ; (if # [cfg ($ i_meta : meta)] { $ ($ i_tokens : tt) * } $ (else if # [cfg ($ e_meta : meta)] { $ ($ e_tokens : tt) * }) *) => { cfg_if ! { @ __items () ; (($ i_meta) ($ ($ i_tokens) *)) , $ ((($ e_meta) ($ ($ e_tokens) *)) ,) * } } ; (if # [cfg ($ i_meta : meta)] { $ ($ i_tokens : tt) * } $ (else if # [cfg ($ e_meta : meta)] { $ ($ e_tokens : tt) * }) * if $ ($ extra_conditions : tt) +) => { cfg_if ! { @ __items () ; (($ i_meta) ($ ($ i_tokens) *)) , $ ((($ e_meta) ($ ($ e_tokens) *)) ,) * } cfg_if ! { if $ ($ extra_conditions) + } } ; (@ __items ($ ($ _ : meta ,) *) ;) => { } ; (@ __items ($ ($ no : meta ,) *) ; (($ ($ yes : meta) ?) ($ ($ tokens : tt) *)) , $ ($ rest : tt ,) *) => { # [cfg (all ($ ($ yes ,) ? not (any ($ ($ no) ,*))))] cfg_if ! { @ __identity $ ($ tokens) * } cfg_if ! { @ __items ($ ($ no ,) * $ ($ yes ,) ?) ; $ ($ rest ,) * } } ; (@ __identity $ ($ tokens : tt) *) => { $ ($ tokens) * } ; }
};
}
