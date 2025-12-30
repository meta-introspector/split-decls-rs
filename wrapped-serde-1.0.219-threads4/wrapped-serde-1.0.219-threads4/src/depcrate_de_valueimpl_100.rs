// Generated macro for impl_100 (impl)
macro_rules! Depcrate_de_valueimpl_100 {
() => {
// Module: crate::de::value
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'de , E > BorrowedBytesDeserializer < 'de , E > { # [doc = " Create a new borrowed deserializer from the given borrowed bytes."] pub fn new (value : & 'de [u8]) -> Self { BorrowedBytesDeserializer { value , marker : PhantomData , } } }
};
}
