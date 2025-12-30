// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl VcpkgTarget { fn link_name_for_lib (& self , filename : & std :: path :: Path) -> Option < String > { if self . target_triplet . strip_lib_prefix { filename . to_str () . map (| s | s . to_owned ()) } else { filename . to_str () . map (| s | s . to_owned ()) } } }
};
}
