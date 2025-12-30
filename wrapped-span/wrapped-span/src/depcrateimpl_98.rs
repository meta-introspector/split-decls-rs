// Generated macro for impl_98 (impl)
macro_rules! Depcrateimpl_98 {
() => {
// Module: crate
// Provides: {"impl_98"}
// Dependencies: {}
impl fmt :: Debug for EditionedFileId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("EditionedFileId") . field (& self . file_id () . index ()) . field (& self . edition ()) . finish () } }
};
}
