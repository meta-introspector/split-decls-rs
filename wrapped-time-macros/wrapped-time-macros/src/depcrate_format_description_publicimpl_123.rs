// Generated macro for impl_123 (impl)
macro_rules! Depcrate_format_description_publicimpl_123 {
() => {
// Module: crate::format_description::public
// Provides: {"impl_123"}
// Dependencies: {}
impl ToTokenStream for OwnedFormatItem { fn append_to (self , ts : & mut TokenStream) { match self { Self :: Literal (bytes) => quote_append ! { ts BorrowedFormatItem :: Literal (# (Literal :: byte_string (bytes . as_ref ()))) } , Self :: Component (component) => quote_append ! { ts BorrowedFormatItem :: Component { 0 : # S (component) } } , Self :: Compound (items) => { let items = items . into_vec () . into_iter () . map (| item | quote_ ! { # S (item) , }) . collect :: < TokenStream > () ; quote_append ! { ts BorrowedFormatItem :: Compound { 0 : & [# S (items)] } } } Self :: Optional (item) => quote_append ! { ts BorrowedFormatItem :: Optional { 0 : &# S (* item) } } , Self :: First (items) => { let items = items . into_vec () . into_iter () . map (| item | quote_ ! { # S (item) , }) . collect :: < TokenStream > () ; quote_append ! { ts BorrowedFormatItem :: First { 0 : & [# S (items)] } } } } } }
};
}
