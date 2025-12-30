// Generated macro for MockClientVerifier (struct)
macro_rules! DepcrateMockClientVerifier {
() => {
// Module: crate
// Provides: {"MockClientVerifier"}
// Dependencies: {}
# [derive (Debug)] pub struct MockClientVerifier { pub verified : fn () -> Result < PeerVerified , Error > , pub subjects : Arc < [DistinguishedName] > , pub mandatory : bool , pub offered_schemes : Option < Vec < SignatureScheme > > , expect_raw_public_keys : bool , raw_public_key_algorithms : Option < WebPkiSupportedAlgorithms > , parent : Arc < dyn ClientVerifier > , }
};
}
