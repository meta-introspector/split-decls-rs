// Generated macro for BorrowedStrDeserializer (struct)
macro_rules! Depcrate_de_valueBorrowedStrDeserializer {
() => {
// Module: crate::de::value
// Provides: {"BorrowedStrDeserializer"}
// Dependencies: {}
# [doc = " A deserializer holding a `&str` with a lifetime tied to another"] # [doc = " deserializer."] pub struct BorrowedStrDeserializer < 'de , E > { value : & 'de str , marker : PhantomData < E > , }
};
}
