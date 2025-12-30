// Generated macro for impl_40 (impl)
macro_rules! Depcrate_delimitedimpl_40 {
() => {
// Module: crate::delimited
// Provides: {"impl_40"}
// Dependencies: {}
impl < T , D > Iterator for IntoIter < T , D > { type Item = Element < T , D > ; fn next (& mut self) -> Option < Element < T , D > > { self . inner . next () . map (| pair | { match pair . 1 { Some (v) => Element :: Delimited (pair . 0 , v) , None => Element :: End (pair . 0) } }) } }
};
}
