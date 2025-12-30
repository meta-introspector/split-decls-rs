// Generated macro for impl_729 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_729 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_729"}
// Dependencies: {}
impl < 'c , Side : SideData > EncodeTlsData < 'c , Side > { fn new (conn : & 'c mut UnbufferedConnectionCommon < Side > , chunk : Vec < u8 >) -> Self { Self { conn , chunk : Some (chunk) , } } # [doc = " Encodes a handshake record into the `outgoing_tls` buffer"] # [doc = ""] # [doc = " Returns the number of bytes that were written into `outgoing_tls`, or an error if"] # [doc = " the provided buffer is too small. In the error case, `outgoing_tls` is not modified"] pub fn encode (& mut self , outgoing_tls : & mut [u8]) -> Result < usize , EncodeError > { let Some (chunk) = self . chunk . take () else { return Err (EncodeError :: AlreadyEncoded) ; } ; let required_size = chunk . len () ; if required_size > outgoing_tls . len () { self . chunk = Some (chunk) ; Err (InsufficientSizeError { required_size } . into ()) } else { let written = chunk . len () ; outgoing_tls [.. written] . copy_from_slice (& chunk) ; self . conn . wants_write = true ; Ok (written) } } }
};
}
