// Generated macro for impl_304 (impl)
macro_rules! Depcrate_typesimpl_304 {
() => {
// Module: crate::types
// Provides: {"impl_304"}
// Dependencies: {}
impl TestName { pub fn as_slice (& self) -> & str { match * self { StaticTestName (s) => s , DynTestName (ref s) => s , AlignedTestName (ref s , _) => s , } } pub fn padding (& self) -> NamePadding { match self { & AlignedTestName (_ , p) => p , _ => PadNone , } } pub fn with_padding (& self , padding : NamePadding) -> TestName { let name = match * self { TestName :: StaticTestName (name) => Cow :: Borrowed (name) , TestName :: DynTestName (ref name) => Cow :: Owned (name . clone ()) , TestName :: AlignedTestName (ref name , _) => name . clone () , } ; TestName :: AlignedTestName (name , padding) } }
};
}
