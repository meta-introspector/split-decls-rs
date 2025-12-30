// Generated macro for UnbufferedStatus (struct)
macro_rules! Depcrate_conn_unbufferedUnbufferedStatus {
() => {
// Module: crate::conn::unbuffered
// Provides: {"UnbufferedStatus"}
// Dependencies: {}
# [doc = " The current status of the `UnbufferedConnection*`"] # [non_exhaustive] # [must_use] # [derive (Debug)] pub struct UnbufferedStatus < 'c , 'i , Data : SideData > { # [doc = " Number of bytes to discard"] # [doc = ""] # [doc = " After the `state` field of this object has been handled, `discard` bytes must be"] # [doc = " removed from the *front* of the `incoming_tls` buffer that was passed to"] # [doc = " the [`UnbufferedConnectionCommon::process_tls_records`] call that returned this object."] # [doc = ""] # [doc = " This discard operation MUST happen *before*"] # [doc = " [`UnbufferedConnectionCommon::process_tls_records`] is called again."] pub discard : usize , # [doc = " The current state of the handshake process"] # [doc = ""] # [doc = " This value MUST be handled prior to calling"] # [doc = " [`UnbufferedConnectionCommon::process_tls_records`] again. See the documentation on the"] # [doc = " variants of [`ConnectionState`] for more details."] pub state : Result < ConnectionState < 'c , 'i , Data > , Error > , }
};
}
