// Generated macro for PreprocessorFSAbstraction (trait)
macro_rules! Depcrate_compiler_cPreprocessorFSAbstraction {
() => {
// Module: crate::compiler::c
// Provides: {"PreprocessorFSAbstraction"}
// Dependencies: {}
# [doc = " An abstraction to filesystem access for use during the preprocessor"] # [doc = " caching phase, to make testing easier."] # [doc = ""] # [doc = " This may help non-local preprocessor caching in the future, if it ends up"] # [doc = " being viable."] trait PreprocessorFSAbstraction { fn metadata (& self , path : impl AsRef < Path >) -> io :: Result < PreprocessorFileMetadata > { std :: fs :: metadata (path) . map (Into :: into) } fn open (& self , path : impl AsRef < Path >) -> io :: Result < Box < dyn std :: io :: Read > > { Ok (Box :: new (std :: fs :: File :: open (path) ?)) } }
};
}
