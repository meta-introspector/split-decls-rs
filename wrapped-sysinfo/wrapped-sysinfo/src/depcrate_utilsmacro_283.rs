// Generated macro for macro_283 (macro)
macro_rules! Depcrate_utilsmacro_283 {
() => {
// Module: crate::utils
// Provides: {"macro_283"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (feature = "multithread" , not (feature = "unknown-ci") , not (all (target_os = "macos" , feature = "apple-sandbox")) ,))] { # [doc = " Converts the value into a parallel iterator if the `multithread` feature is enabled."] # [doc = " Uses the `rayon::iter::IntoParallelIterator` trait."] # [cfg (all (feature = "multithread" , not (feature = "unknown-ci") , not (all (target_os = "macos" , feature = "apple-sandbox")) ,))] # [allow (dead_code)] pub (crate) fn into_iter < T > (val : T) -> T :: Iter where T : rayon :: iter :: IntoParallelIterator , { val . into_par_iter () } } else { # [doc = " Converts the value into a sequential iterator if the `multithread` feature is disabled."] # [doc = " Uses the `std::iter::IntoIterator` trait."] # [allow (dead_code)] pub (crate) fn into_iter < T > (val : T) -> T :: IntoIter where T : IntoIterator , { val . into_iter () } } }
};
}
