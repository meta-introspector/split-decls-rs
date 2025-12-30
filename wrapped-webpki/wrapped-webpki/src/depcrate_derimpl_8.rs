// Generated macro for impl_8 (impl)
macro_rules! Depcrate_derimpl_8 {
() => {
// Module: crate::der
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a , T > DerIterator < 'a , T > { # [doc = " [`DerIterator`] will consume all of the bytes in `input` reading values of type `T`."] pub (crate) fn new (input : untrusted :: Input < 'a >) -> Self { Self { reader : untrusted :: Reader :: new (input) , marker : PhantomData , } } }
};
}
