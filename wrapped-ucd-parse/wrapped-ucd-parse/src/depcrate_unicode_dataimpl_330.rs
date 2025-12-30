// Generated macro for impl_330 (impl)
macro_rules! Depcrate_unicode_dataimpl_330 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_330"}
// Dependencies: {}
impl std :: fmt :: Display for UnicodeDataDecomposition { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if let Some (ref tag) = self . tag { write ! (f , "<{}> " , tag) ? ; } let mut first = true ; for cp in self . mapping () { if ! first { write ! (f , " ") ? ; } first = false ; write ! (f , "{}" , cp) ? ; } Ok (()) } }
};
}
