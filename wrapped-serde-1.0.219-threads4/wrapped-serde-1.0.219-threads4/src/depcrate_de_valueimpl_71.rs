// Generated macro for impl_71 (impl)
macro_rules! Depcrate_de_valueimpl_71 {
() => {
// Module: crate::de::value
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'de , E > BorrowedStrDeserializer < 'de , E > { # [doc = " Create a new borrowed deserializer from the given string."] pub fn new (value : & 'de str) -> BorrowedStrDeserializer < 'de , E > { BorrowedStrDeserializer { value , marker : PhantomData , } } }
};
}
