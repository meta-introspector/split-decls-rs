// Generated macro for ConnectionCommon (struct)
macro_rules! Depcrate_connConnectionCommon {
() => {
// Module: crate::conn
// Provides: {"ConnectionCommon"}
// Dependencies: {}
# [doc = " TLS connection state with side-specific data (`Side`)."] # [doc = ""] # [doc = " This is one of the core abstractions of the rustls API. It represents a single connection"] # [doc = " to a peer, and holds all the state associated with that connection. Note that it does"] # [doc = " not hold any IO objects: the application is responsible for reading and writing TLS records."] # [doc = " If you want an object that does hold IO objects, see [`Stream`] and [`StreamOwned`]."] # [doc = ""] # [doc = " This object is generic over the `Side` type parameter, which must implement the marker trait"] # [doc = " [`SideData`]. This is used to store side-specific data."] # [doc = ""] # [doc = " [`Stream`]: crate::Stream"] # [doc = " [`StreamOwned`]: crate::StreamOwned"] pub struct ConnectionCommon < Side : SideData > { pub (crate) core : ConnectionCore < Side > , deframer_buffer : DeframerVecBuffer , sendable_plaintext : ChunkVecBuffer , }
};
}
