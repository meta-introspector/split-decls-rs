// Generated macro for record_variants (macro)
macro_rules! Depcrate_input_statsrecord_variants {
() => {
// Module: crate::input_stats
// Provides: {"record_variants"}
// Dependencies: {}
macro_rules ! record_variants { (($ self : ident , $ val : expr , $ kind : expr , $ id : expr , $ mod : ident , $ ty : ty , $ tykind : ident) , [$ ($ variant : ident) ,*]) => { match $ kind { $ ($ mod ::$ tykind ::$ variant { .. } => { $ self . record_variant (stringify ! ($ ty) , stringify ! ($ variant) , $ id , $ val) }) * } } ; }
};
}
