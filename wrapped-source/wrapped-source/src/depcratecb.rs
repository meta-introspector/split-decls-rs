// Generated macro for cb (macro)
macro_rules! Depcratecb {
() => {
// Module: crate
// Provides: {"cb"}
// Dependencies: {}
macro_rules ! cb { ($ ($ marker_ty : ty :$ marker : ident ,) + # [experimental] $ ($ emarker_ty : ty :$ emarker : ident ,) +) => { icu_provider :: export :: make_exportable_provider ! (SourceDataProvider , [$ ($ marker_ty ,) + $ (# [cfg (feature = "experimental")] $ emarker_ty ,) +]) ; } }
};
}
