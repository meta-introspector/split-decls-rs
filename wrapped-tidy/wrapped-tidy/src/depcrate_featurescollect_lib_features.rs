// Generated macro for collect_lib_features (function)
macro_rules! Depcrate_featurescollect_lib_features {
() => {
// Module: crate::features
// Provides: {"collect_lib_features"}
// Dependencies: {}
pub fn collect_lib_features (base_src_path : & Path) -> Features { let mut lib_features = Features :: new () ; map_lib_features (base_src_path , & mut | res , _ , _ | { if let Ok ((name , feature)) = res { lib_features . insert (name . to_owned () , feature) ; } }) ; lib_features }
};
}
