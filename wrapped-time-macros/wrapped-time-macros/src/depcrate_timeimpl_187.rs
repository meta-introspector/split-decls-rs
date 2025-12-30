// Generated macro for impl_187 (impl)
macro_rules! Depcrate_timeimpl_187 {
() => {
// Module: crate::time
// Provides: {"impl_187"}
// Dependencies: {}
impl ToTokenStream for Time { fn append_to (self , ts : & mut TokenStream) { quote_append ! { ts unsafe { :: time :: Time :: __from_hms_nanos_unchecked (# (self . hour) , # (self . minute) , # (self . second) , # (self . nanosecond) ,) } } } }
};
}
