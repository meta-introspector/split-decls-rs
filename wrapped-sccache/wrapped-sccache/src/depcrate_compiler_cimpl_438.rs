// Generated macro for impl_438 (impl)
macro_rules! Depcrate_compiler_cimpl_438 {
() => {
// Module: crate::compiler::c
// Provides: {"impl_438"}
// Dependencies: {}
impl From < std :: fs :: Metadata > for PreprocessorFileMetadata { fn from (meta : std :: fs :: Metadata) -> Self { Self { is_dir : meta . is_dir () , is_file : meta . is_file () , modified : meta . modified () . ok () . map (Into :: into) , ctime_or_creation : meta . ctime_or_creation () . ok () , } } }
};
}
