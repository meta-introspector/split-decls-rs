// Generated macro for impl_36 (impl)
macro_rules! Depcrate_delimitedimpl_36 {
() => {
// Module: crate::delimited
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a , T , D > Iterator for IterMut < 'a , T , D > { type Item = Element < & 'a mut T , & 'a mut D > ; fn next (& mut self) -> Option < Element < & 'a mut T , & 'a mut D > > { self . inner . next () . map (| pair | { match pair . 1 { Some (ref mut delimited) => Element :: Delimited (& mut pair . 0 , delimited) , None => Element :: End (& mut pair . 0) , } }) } }
};
}
