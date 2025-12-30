// Generated macro for unpad_tls13_payload (function)
macro_rules! Depcrate_crypto_cipher_inboundunpad_tls13_payload {
() => {
// Module: crate::crypto::cipher::inbound
// Provides: {"unpad_tls13_payload"}
// Dependencies: {}
# [doc = " Decode a TLS1.3 `TLSInnerPlaintext` encoding."] # [doc = ""] # [doc = " `p` is a message payload, immediately post-decryption.  This function"] # [doc = " removes zero padding bytes, until a non-zero byte is encountered which is"] # [doc = " the content type, which is returned.  See RFC8446 s5.2."] # [doc = ""] # [doc = " ContentType(0) is returned if the message payload is empty or all zeroes."] fn unpad_tls13_payload (p : & mut BorrowedPayload < '_ >) -> ContentType { loop { match p . pop () { Some (0) => { } Some (content_type) => return ContentType :: from (content_type) , None => return ContentType :: Unknown (0) , } } }
};
}
