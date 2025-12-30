// Generated macro for impl_173 (impl)
macro_rules! Depcrate_offsetimpl_173 {
() => {
// Module: crate::offset
// Provides: {"impl_173"}
// Dependencies: {}
impl ToTokenStream for Offset { fn append_to (self , ts : & mut TokenStream) { quote_append ! { ts unsafe { :: time :: UtcOffset :: __from_hms_unchecked (# (self . hours) , # (self . minutes) , # (self . seconds) ,) } } } }
};
}
