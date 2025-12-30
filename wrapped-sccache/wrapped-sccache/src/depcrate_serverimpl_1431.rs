// Generated macro for impl_1431 (impl)
macro_rules! Depcrate_serverimpl_1431 {
() => {
// Module: crate::server
// Provides: {"impl_1431"}
// Dependencies: {}
impl < T > tokio_serde :: Serializer < T > for BincodeCodec where T : serde :: Serialize , { type Error = Error ; fn serialize (self : Pin < & mut Self > , item : & T) -> std :: result :: Result < Bytes , Self :: Error > { let mut bytes = BytesMut :: new () ; bincode :: serialize_into ((& mut bytes) . writer () , item) ? ; Ok (bytes . freeze ()) } }
};
}
