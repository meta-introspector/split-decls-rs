// Generated macro for impl_1432 (impl)
macro_rules! Depcrate_serverimpl_1432 {
() => {
// Module: crate::server
// Provides: {"impl_1432"}
// Dependencies: {}
impl < T > tokio_serde :: Deserializer < T > for BincodeCodec where T : serde :: de :: DeserializeOwned , { type Error = Error ; fn deserialize (self : Pin < & mut Self > , buf : & BytesMut) -> std :: result :: Result < T , Self :: Error > { let ret = bincode :: deserialize (buf) ? ; Ok (ret) } }
};
}
