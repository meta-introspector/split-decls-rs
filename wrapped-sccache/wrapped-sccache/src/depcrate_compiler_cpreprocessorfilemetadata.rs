// Generated macro for PreprocessorFileMetadata (struct)
macro_rules! Depcrate_compiler_cPreprocessorFileMetadata {
() => {
// Module: crate::compiler::c
// Provides: {"PreprocessorFileMetadata"}
// Dependencies: {}
# [doc = " Limited abstraction of `std::fs::Metadata`, allowing us to create fake"] # [doc = " values during testing."] # [derive (Debug , Eq , PartialEq , Clone)] struct PreprocessorFileMetadata { is_dir : bool , is_file : bool , modified : Option < Timestamp > , ctime_or_creation : Option < Timestamp > , }
};
}
