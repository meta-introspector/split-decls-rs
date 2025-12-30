// Generated macro for Algorithm (trait)
macro_rules! Depcrate_quicAlgorithm {
() => {
// Module: crate::quic
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " How a `Tls13CipherSuite` generates `PacketKey`s and `HeaderProtectionKey`s."] pub trait Algorithm : Send + Sync { # [doc = " Produce a `PacketKey` encrypter/decrypter for this suite."] # [doc = ""] # [doc = " `suite` is the entire suite this `Algorithm` appeared in."] # [doc = " `key` and `iv` is the key material to use."] fn packet_key (& self , key : AeadKey , iv : Iv) -> Box < dyn PacketKey > ; # [doc = " Produce a `HeaderProtectionKey` encrypter/decrypter for this suite."] # [doc = ""] # [doc = " `key` is the key material, which is `aead_key_len()` bytes in length."] fn header_protection_key (& self , key : AeadKey) -> Box < dyn HeaderProtectionKey > ; # [doc = " The length in bytes of keys for this Algorithm."] # [doc = ""] # [doc = " This controls the size of `AeadKey`s presented to `packet_key()` and `header_protection_key()`."] fn aead_key_len (& self) -> usize ; # [doc = " Whether this algorithm is FIPS-approved."] fn fips (& self) -> bool { false } }
};
}
