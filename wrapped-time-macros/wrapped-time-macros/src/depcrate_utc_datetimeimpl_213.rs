// Generated macro for impl_213 (impl)
macro_rules! Depcrate_utc_datetimeimpl_213 {
() => {
// Module: crate::utc_datetime
// Provides: {"impl_213"}
// Dependencies: {}
impl ToTokenStream for UtcDateTime { fn append_to (self , ts : & mut TokenStream) { quote_append ! { ts :: time :: UtcDateTime :: new (# S (self . date) , # S (self . time) ,) } } }
};
}
