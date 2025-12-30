// Generated macro for impl_649 (impl)
macro_rules! Depcrate_datetime_neo_skeletonimpl_649 {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"impl_649"}
// Dependencies: {}
impl DataProvider < DatetimePatternsTimeV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < DatetimePatternsTimeV1 > , DataError > { self . load_neo_skeletons_key (req , None , gen_time_components) } }
};
}
