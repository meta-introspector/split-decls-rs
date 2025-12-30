// Generated macro for read_hashes (function)
macro_rules! Depcrate_binary_formatread_hashes {
() => {
// Module: crate::binary_format
// Provides: {"read_hashes"}
// Dependencies: {}
# [doc = " Read a slice of clause hashes from a varisat proof"] fn read_hashes (source : & mut impl BufRead , hashes : & mut Vec < ClauseHash >) -> Result < () , io :: Error > { hashes . clear () ; let len = read_u64 (& mut * source) ? as usize ; hashes . reserve (len) ; for _ in 0 .. len { hashes . push (read_u64 (& mut * source) ? as ClauseHash) ; } Ok (()) }
};
}
