// Generated macro for impl_35 (impl)
macro_rules! Depcrate_data_tzifimpl_35 {
() => {
// Module: crate::data::tzif
// Provides: {"impl_35"}
// Dependencies: {}
impl DataBlock { # [doc = " Retrieves the timezone designation at index `idx`."] pub fn time_zone_designation (& self , mut idx : usize) -> Option < & str > { self . time_zone_designations . iter () . find_map (| d | { if idx <= d . len () { Some (& d [idx ..]) } else { idx -= d . len () + 1 ; None } }) } }
};
}
