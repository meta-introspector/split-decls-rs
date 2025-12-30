// Generated macro for impl_20 (impl)
macro_rules! Depcrate_dateimpl_20 {
() => {
// Module: crate::date
// Provides: {"impl_20"}
// Dependencies: {}
impl ToTokenStream for Date { fn append_to (self , ts : & mut proc_macro :: TokenStream) { quote_append ! { ts unsafe { :: time :: Date :: __from_ordinal_date_unchecked (# (self . year) , # (self . ordinal) ,) } } } }
};
}
