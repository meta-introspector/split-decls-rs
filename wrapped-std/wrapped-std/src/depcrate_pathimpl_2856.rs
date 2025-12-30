// Generated macro for impl_2856 (impl)
macro_rules! Depcrate_pathimpl_2856 {
() => {
// Module: crate::path
// Provides: {"impl_2856"}
// Dependencies: {}
# [stable (feature = "path_from_str" , since = "1.32.0")] impl FromStr for PathBuf { type Err = core :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (PathBuf :: from (s)) } }
};
}
