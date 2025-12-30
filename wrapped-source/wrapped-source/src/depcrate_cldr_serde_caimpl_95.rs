// Generated macro for impl_95 (impl)
macro_rules! Depcrate_cldr_serde_caimpl_95 {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"impl_95"}
// Dependencies: {}
impl DateTimeFormatsVariant { pub (crate) fn get_pattern (& self , length : PatternLength) -> & LengthPattern { match length { PatternLength :: Long => & self . standard . long , PatternLength :: Medium => & self . standard . medium , PatternLength :: Short => & self . standard . short , } } }
};
}
