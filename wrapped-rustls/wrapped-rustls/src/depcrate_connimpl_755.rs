// Generated macro for impl_755 (impl)
macro_rules! Depcrate_connimpl_755 {
() => {
// Module: crate::conn
// Provides: {"impl_755"}
// Dependencies: {}
impl < Side : SideData > From < ConnectionCore < Side > > for ConnectionCommon < Side > { fn from (core : ConnectionCore < Side >) -> Self { Self { core , deframer_buffer : DeframerVecBuffer :: default () , sendable_plaintext : ChunkVecBuffer :: new (Some (DEFAULT_BUFFER_LIMIT)) , } } }
};
}
