// Generated macro for impl_28 (impl)
macro_rules! Depcrate_delimitedimpl_28 {
() => {
// Module: crate::delimited
// Provides: {"impl_28"}
// Dependencies: {}
impl < T , D > Extend < Element < T , D > > for Delimited < T , D > { fn extend < I : IntoIterator < Item = Element < T , D > > > (& mut self , i : I) { for element in i { match element { Element :: Delimited (a , b) => self . inner . push ((a , Some (b))) , Element :: End (a) => self . inner . push ((a , None)) , } } } }
};
}
