// Generated macro for impl_is_terminal (macro)
macro_rules! Depcrate_io_stdioimpl_is_terminal {
() => {
// Module: crate::io::stdio
// Provides: {"impl_is_terminal"}
// Dependencies: {}
macro_rules ! impl_is_terminal { ($ ($ t : ty) ,*$ (,) ?) => { $ (# [unstable (feature = "sealed" , issue = "none")] impl crate :: sealed :: Sealed for $ t { } # [stable (feature = "is_terminal" , since = "1.70.0")] impl IsTerminal for $ t { # [inline] fn is_terminal (& self) -> bool { crate :: sys :: io :: is_terminal (self) } }) * } }
};
}
