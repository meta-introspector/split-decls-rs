// Generated macro for impl_552 (impl)
macro_rules! Depcrate_compiler_compilerimpl_552 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_552"}
// Dependencies: {}
impl Language { pub fn from_file_name (file : & Path) -> Option < Self > { match file . extension () . and_then (| e | e . to_str ()) { Some ("c") => Some (Language :: C) , Some ("h") => Some (Language :: GenericHeader) , Some ("C") | Some ("cc") | Some ("cp") | Some ("cpp") | Some ("CPP") | Some ("cxx") | Some ("c++") => Some (Language :: Cxx) , Some ("H") | Some ("hh") | Some ("hp") | Some ("hpp") | Some ("HPP") | Some ("hxx") | Some ("h++") | Some ("tcc") => Some (Language :: CxxHeader) , Some ("m") => Some (Language :: ObjectiveC) , Some ("M") | Some ("mm") => Some (Language :: ObjectiveCxx) , Some ("cu") => Some (Language :: Cuda) , Some ("ptx") => Some (Language :: Ptx) , Some ("cubin") => Some (Language :: Cubin) , Some ("rs") => Some (Language :: Rust) , Some ("hip") => Some (Language :: Hip) , e => { trace ! ("Unknown source extension: {}" , e . unwrap_or ("(None)")) ; None } } } pub fn as_str (self) -> & 'static str { match self { Language :: C => "c" , Language :: CHeader => "cHeader" , Language :: Cxx => "c++" , Language :: CxxHeader => "c++Header" , Language :: GenericHeader => "c/c++" , Language :: ObjectiveC => "objc" , Language :: ObjectiveCxx | Language :: ObjectiveCxxHeader => "objc++" , Language :: Cuda => "cuda" , Language :: CudaFE => "cuda" , Language :: Ptx => "ptx" , Language :: Cubin => "cubin" , Language :: Rust => "rust" , Language :: Hip => "hip" , } } }
};
}
