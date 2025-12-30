// Generated macro for declare_tool_lint (macro)
macro_rules! Depcratedeclare_tool_lint {
() => {
// Module: crate
// Provides: {"declare_tool_lint"}
// Dependencies: {}
# [macro_export] macro_rules ! declare_tool_lint { ($ (# [$ attr : meta]) * $ vis : vis $ tool : ident ::$ NAME : ident , $ Level : ident , $ desc : expr $ (, @ eval_always = $ eval_always : literal) ? $ (, @ feature_gate = $ gate : ident ;) ?) => ($ crate :: declare_tool_lint ! { $ (# [$ attr]) * $ vis $ tool ::$ NAME , $ Level , $ desc , false $ (, @ eval_always = $ eval_always) ? $ (, @ feature_gate = $ gate ;) ? }) ; ($ (# [$ attr : meta]) * $ vis : vis $ tool : ident ::$ NAME : ident , $ Level : ident , $ desc : expr , report_in_external_macro : $ rep : expr $ (, @ eval_always = $ eval_always : literal) ? $ (, @ feature_gate = $ gate : ident ;) ?) => ($ crate :: declare_tool_lint ! { $ (# [$ attr]) * $ vis $ tool ::$ NAME , $ Level , $ desc , $ rep $ (, @ eval_always = $ eval_always) ? $ (, @ feature_gate = $ gate ;) ? }) ; ($ (# [$ attr : meta]) * $ vis : vis $ tool : ident ::$ NAME : ident , $ Level : ident , $ desc : expr , $ external : expr $ (, @ eval_always = $ eval_always : literal) ? $ (, @ feature_gate = $ gate : ident ;) ?) => ($ (# [$ attr]) * $ vis static $ NAME : &$ crate :: Lint = &$ crate :: Lint { name : & concat ! (stringify ! ($ tool) , "::" , stringify ! ($ NAME)) , default_level : $ crate ::$ Level , desc : $ desc , edition_lint_opts : None , report_in_external_macro : $ external , future_incompatible : None , is_externally_loaded : true , $ (feature_gate : Some (rustc_span :: sym ::$ gate) ,) ? crate_level_only : false , $ (eval_always : $ eval_always ,) ? ..$ crate :: Lint :: default_fields_for_macro () } ;) ; }
};
}
