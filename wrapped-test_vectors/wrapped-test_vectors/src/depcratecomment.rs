// Generated macro for COMMENT (const)
macro_rules! DepcrateCOMMENT {
() => {
// Module: crate
// Provides: {"COMMENT"}
// Dependencies: {}
const COMMENT : & str = r#"
Each test is an input length and three outputs, one for each of the hash,
keyed_hash, and derive_key modes. The input in each case is filled with a
repeating sequence of 251 bytes: 0, 1, 2, ..., 249, 250, 0, 1, ..., and so on.
The key used with keyed_hash is the 32-byte ASCII string "whats the Elvish word
for friend", also given in the `key` field below. The context string used with
derive_key is the ASCII string "BLAKE3 2019-12-27 16:29:52 test vectors
context", also given in the `context_string` field below. Outputs are encoded
as hexadecimal. Each case is an extended output, and implementations should
also check that the first 32 bytes match their default-length output.
"# ;
};
}
