// Generated macro for gen_time_components (function)
macro_rules! Depcrate_datetime_neo_skeletongen_time_components {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"gen_time_components"}
// Dependencies: {}
# [doc = " Convert from a semantic time field set to classical component options for calculating the pattern."] fn gen_time_components (_ : Length , attributes : & DataMarkerAttributes , _ : & cldr_serde :: ca :: Dates ,) -> components :: Bag { let mut filtered_components = components :: Bag :: empty () ; filtered_components . hour = Some (components :: Numeric :: Numeric) ; if check_for_field (attributes , "h") { filtered_components . hour_cycle = Some (HourCycle :: H12) ; } if check_for_field (attributes , "h0") { filtered_components . hour_cycle = Some (HourCycle :: H23) ; } filtered_components }
};
}
