// Generated macro for impl_2880 (impl)
macro_rules! Depcrate_pathimpl_2880 {
() => {
// Module: crate::path
// Provides: {"impl_2880"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Hash for PathBuf { fn hash < H : Hasher > (& self , h : & mut H) { self . as_path () . hash (h) } }
};
}
