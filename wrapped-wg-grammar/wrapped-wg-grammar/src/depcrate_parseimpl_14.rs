// Generated macro for impl_14 (impl)
macro_rules! Depcrate_parseimpl_14 {
() => {
// Module: crate::parse
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input , T : ? Sized > Handle < 'a , 'i , I , T > { pub fn source (self) -> & 'a I :: Slice { self . parser . input (self . node . range) } pub fn source_info (self) -> I :: SourceInfo { self . parser . source_info (self . node . range) } }
};
}
