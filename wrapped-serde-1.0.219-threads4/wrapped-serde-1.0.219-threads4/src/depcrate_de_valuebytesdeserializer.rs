// Generated macro for BytesDeserializer (struct)
macro_rules! Depcrate_de_valueBytesDeserializer {
() => {
// Module: crate::de::value
// Provides: {"BytesDeserializer"}
// Dependencies: {}
# [doc = " A deserializer holding a `&[u8]`. Always calls [`Visitor::visit_bytes`]."] pub struct BytesDeserializer < 'a , E > { value : & 'a [u8] , marker : PhantomData < E > , }
};
}
