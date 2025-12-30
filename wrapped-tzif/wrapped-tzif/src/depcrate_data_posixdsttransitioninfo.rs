// Generated macro for DstTransitionInfo (struct)
macro_rules! Depcrate_data_posixDstTransitionInfo {
() => {
// Module: crate::data::posix
// Provides: {"DstTransitionInfo"}
// Dependencies: {}
# [doc = " A struct for holding DST transition info."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct DstTransitionInfo { # [doc = " The zone variant info including name and offset."] pub variant_info : TimeZoneVariantInfo , # [doc = " The DST transition start date."] pub start_date : TransitionDate , # [doc = " The DST transition end date."] pub end_date : TransitionDate , }
};
}
