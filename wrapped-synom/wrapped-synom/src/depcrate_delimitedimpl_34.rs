// Generated macro for impl_34 (impl)
macro_rules! Depcrate_delimitedimpl_34 {
() => {
// Module: crate::delimited
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , T , D > Iterator for Iter < 'a , T , D > { type Item = Element < & 'a T , & 'a D > ; fn next (& mut self) -> Option < Element < & 'a T , & 'a D > > { self . inner . next () . map (| pair | { match pair . 1 { Some (ref delimited) => Element :: Delimited (& pair . 0 , delimited) , None => Element :: End (& pair . 0) , } }) } }
};
}
