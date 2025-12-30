// Generated macro for impl_531 (impl)
macro_rules! Depcrate_testsimpl_531 {
() => {
// Module: crate::tests
// Provides: {"impl_531"}
// Dependencies: {}
# [cfg (all (test , not (feature = "std")))] impl crate :: io_nostd :: Read for std :: fs :: File { fn read (& mut self , buf : & mut [u8]) -> Result < usize , crate :: io_nostd :: Error > { std :: io :: Read :: read (self , buf) . map_err (| e | { if e . get_ref () . is_none () { crate :: io_nostd :: Error :: from (crate :: io_nostd :: ErrorKind :: Other) } else { crate :: io_nostd :: Error :: new (crate :: io_nostd :: ErrorKind :: Other , alloc :: boxed :: Box :: new (e . into_inner () . unwrap ()) ,) } }) } }
};
}
