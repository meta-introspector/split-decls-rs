// Generated macro for impl_97 (impl)
macro_rules! Depcrateimpl_97 {
() => {
// Module: crate
// Provides: {"impl_97"}
// Dependencies: {}
impl fmt :: Debug for EditionedFileId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("EditionedFileId") . field (& self . file_id () . index ()) . field (& self . edition ()) . finish () } }
};
}
