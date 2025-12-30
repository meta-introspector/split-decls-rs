// Generated macro for DerIterator (struct)
macro_rules! Depcrate_derDerIterator {
() => {
// Module: crate::der
// Provides: {"DerIterator"}
// Dependencies: {}
# [doc = " Iterator to parse a sequence of DER-encoded values of type `T`."] # [derive (Debug)] pub struct DerIterator < 'a , T > { reader : untrusted :: Reader < 'a > , marker : PhantomData < T > , }
};
}
