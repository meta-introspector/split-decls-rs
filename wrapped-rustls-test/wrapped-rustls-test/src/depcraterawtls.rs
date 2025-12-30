// Generated macro for RawTls (struct)
macro_rules! DepcrateRawTls {
() => {
// Module: crate
// Provides: {"RawTls"}
// Dependencies: {}
# [doc = " This allows injection/receipt of raw messages into a post-handshake connection."] # [doc = ""] # [doc = " It consumes one of the peers, extracts its secrets, and then reconstitutes the"] # [doc = " message encrypter/decrypter.  It does not do fragmentation/joining."] pub struct RawTls { encrypter : Box < dyn MessageEncrypter > , enc_seq : u64 , decrypter : Box < dyn MessageDecrypter > , dec_seq : u64 , }
};
}
