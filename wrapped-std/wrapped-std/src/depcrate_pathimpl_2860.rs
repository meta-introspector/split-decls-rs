// Generated macro for impl_2860 (impl)
macro_rules! Depcrate_pathimpl_2860 {
() => {
// Module: crate::path
// Provides: {"impl_2860"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ops :: Deref for PathBuf { type Target = Path ; # [inline] fn deref (& self) -> & Path { Path :: new (& self . inner) } }
};
}
