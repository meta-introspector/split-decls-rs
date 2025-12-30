// Generated macro for language_to_gcc_arg (function)
macro_rules! Depcrate_compiler_gcclanguage_to_gcc_arg {
() => {
// Module: crate::compiler::gcc
// Provides: {"language_to_gcc_arg"}
// Dependencies: {}
pub fn language_to_gcc_arg (lang : Language) -> Option < & 'static str > { match lang { Language :: C => Some ("c") , Language :: CHeader => Some ("c-header") , Language :: Cxx => Some ("c++") , Language :: CxxHeader => Some ("c++-header") , Language :: ObjectiveC => Some ("objective-c") , Language :: ObjectiveCxx => Some ("objective-c++") , Language :: ObjectiveCxxHeader => Some ("objective-c++-header") , Language :: Cuda => Some ("cu") , Language :: CudaFE => None , Language :: Ptx => None , Language :: Cubin => None , Language :: Rust => None , Language :: Hip => Some ("hip") , Language :: GenericHeader => None , } }
};
}
