// Generated macro for language_to_clang_arg (function)
macro_rules! Depcrate_compiler_clanglanguage_to_clang_arg {
() => {
// Module: crate::compiler::clang
// Provides: {"language_to_clang_arg"}
// Dependencies: {}
pub fn language_to_clang_arg (lang : Language) -> Option < & 'static str > { match lang { Language :: C => Some ("c") , Language :: CHeader => Some ("c-header") , Language :: Cxx => Some ("c++") , Language :: CxxHeader => Some ("c++-header") , Language :: ObjectiveC => Some ("objective-c") , Language :: ObjectiveCxx => Some ("objective-c++") , Language :: ObjectiveCxxHeader => Some ("objective-c++-header") , Language :: Cuda => Some ("cuda") , Language :: CudaFE => None , Language :: Ptx => None , Language :: Cubin => None , Language :: Rust => None , Language :: Hip => Some ("hip") , Language :: GenericHeader => None , } }
};
}
