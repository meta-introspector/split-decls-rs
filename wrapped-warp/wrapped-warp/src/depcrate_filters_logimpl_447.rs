// Generated macro for impl_447 (impl)
macro_rules! Depcrate_filters_logimpl_447 {
() => {
// Module: crate::filters::log
// Provides: {"impl_447"}
// Dependencies: {}
impl < T : fmt :: Display > fmt :: Display for OptFmt < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref t) = self . 0 { fmt :: Display :: fmt (t , f) } else { f . write_str ("-") } } }
};
}
