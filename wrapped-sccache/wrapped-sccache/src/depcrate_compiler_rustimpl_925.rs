// Generated macro for impl_925 (impl)
macro_rules! Depcrate_compiler_rustimpl_925 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_925"}
// Dependencies: {}
# [cfg (feature = "dist-client")] impl Meter < PathBuf , RlibDepsDetail > for DepsSize { type Measure = usize ; fn measure < Q : ? Sized > (& self , _k : & Q , v : & RlibDepsDetail) -> usize where PathBuf : Borrow < Q > , { use std :: mem ; let k_size = 3 * 8 + 100 ; let crate_names_size : usize = v . deps . iter () . map (| s | s . capacity ()) . sum () ; let v_size : usize = mem :: size_of :: < RlibDepsDetail > () + v . deps . capacity () * mem :: size_of :: < String > () + crate_names_size ; k_size + v_size } }
};
}
