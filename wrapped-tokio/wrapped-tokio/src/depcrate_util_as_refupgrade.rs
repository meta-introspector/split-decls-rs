// Generated macro for upgrade (function)
macro_rules! Depcrate_util_as_refupgrade {
() => {
// Module: crate::util::as_ref
// Provides: {"upgrade"}
// Dependencies: {}
pub (crate) fn upgrade < B : AsRef < [u8] > > (buf : B) -> OwnedBuf { let buf = match unsafe { typeid :: try_transmute :: < B , Vec < u8 > > (buf) } { Ok (vec) => return OwnedBuf :: Vec (vec) , Err (original_buf) => original_buf , } ; let buf = match unsafe { typeid :: try_transmute :: < B , String > (buf) } { Ok (string) => return OwnedBuf :: Vec (string . into_bytes ()) , Err (original_buf) => original_buf , } ; # [cfg (feature = "io-util")] let buf = match unsafe { typeid :: try_transmute :: < B , bytes :: Bytes > (buf) } { Ok (bytes) => return OwnedBuf :: Bytes (bytes) , Err (original_buf) => original_buf , } ; OwnedBuf :: Vec (buf . as_ref () . to_owned ()) }
};
}
