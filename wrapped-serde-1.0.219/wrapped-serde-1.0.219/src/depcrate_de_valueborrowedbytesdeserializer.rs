// Generated macro for BorrowedBytesDeserializer (struct)
macro_rules! Depcrate_de_valueBorrowedBytesDeserializer {
() => {
// Module: crate::de::value
// Provides: {"BorrowedBytesDeserializer"}
// Dependencies: {}
# [doc = " A deserializer holding a `&[u8]` with a lifetime tied to another"] # [doc = " deserializer. Always calls [`Visitor::visit_borrowed_bytes`]."] pub struct BorrowedBytesDeserializer < 'de , E > { value : & 'de [u8] , marker : PhantomData < E > , }
};
}
