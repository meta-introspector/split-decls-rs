// Generated macro for Digest (struct)
macro_rules! DepcrateDigest {
() => {
// Module: crate
// Provides: {"Digest"}
// Dependencies: {}
# [doc = " Digest generated from a `Sha1` instance."] # [doc = ""] # [doc = " A digest can be formatted to view the digest as a hex string, or the bytes"] # [doc = " can be extracted for later processing."] # [doc = ""] # [doc = " To retrieve a hex string result call `to_string` on it (requires that std"] # [doc = " is available)."] # [doc = ""] # [doc = " If the `serde` feature is enabled a digest can also be serialized and"] # [doc = " deserialized.  Likewise a digest can be parsed from a hex string."] # [derive (PartialOrd , Ord , PartialEq , Eq , Hash , Clone , Copy , Default)] pub struct Digest { data : Sha1State , }
};
}
