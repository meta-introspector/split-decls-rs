// Generated macro for impl_2874 (impl)
macro_rules! Depcrate_pathimpl_2874 {
() => {
// Module: crate::path
// Provides: {"impl_2874"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToOwned for Path { type Owned = PathBuf ; # [inline] fn to_owned (& self) -> PathBuf { self . to_path_buf () } # [inline] fn clone_into (& self , target : & mut PathBuf) { self . inner . clone_into (& mut target . inner) ; } }
};
}
