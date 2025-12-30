// Generated macro for impl_151 (impl)
macro_rules! Depcrate_tinyvecimpl_151 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docs_rs , doc (cfg (feature = "std")))] impl < A : Array < Item = u8 > > std :: io :: Write for TinyVec < A > { # [inline (always)] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline (always)] fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
