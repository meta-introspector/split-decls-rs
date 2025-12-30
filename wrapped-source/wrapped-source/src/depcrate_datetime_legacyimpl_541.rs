// Generated macro for impl_541 (impl)
macro_rules! Depcrate_datetime_legacyimpl_541 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_541"}
// Dependencies: {}
impl From < & cldr_serde :: ca :: Dates > for DateLengths < '_ > { fn from (other : & cldr_serde :: ca :: Dates) -> Self { let length_combinations_v1 = GenericLengthPatterns :: from (& other . datetime_formats_at_time) ; Self { date : (& other . date_skeletons) . into () , length_combinations : length_combinations_v1 , } } }
};
}
