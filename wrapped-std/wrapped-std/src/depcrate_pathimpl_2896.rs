// Generated macro for impl_2896 (impl)
macro_rules! Depcrate_pathimpl_2896 {
() => {
// Module: crate::path
// Provides: {"impl_2896"}
// Dependencies: {}
# [stable (feature = "eq_str_for_path" , since = "1.91.0")] impl cmp :: PartialEq < str > for Path { # [inline] fn eq (& self , other : & str) -> bool { let other : & OsStr = other . as_ref () ; self == other } }
};
}
