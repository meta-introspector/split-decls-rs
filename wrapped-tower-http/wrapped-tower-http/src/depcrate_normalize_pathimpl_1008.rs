// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_normalize_pathimpl_1008 {
() => {
// Module: crate::normalize_path
// Provides: {"impl_1008"}
// Dependencies: {}
impl < S > NormalizePath < S > { # [doc = " Construct a new [`NormalizePath`] with trim mode."] pub fn trim_trailing_slash (inner : S) -> Self { Self { mode : NormalizeMode :: Trim , inner , } } # [doc = " Construct a new [`NormalizePath`] with append mode."] pub fn append_trailing_slash (inner : S) -> Self { Self { mode : NormalizeMode :: Append , inner , } } define_inner_service_accessors ! () ; }
};
}
