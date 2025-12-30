// Generated macro for write_hashes (function)
macro_rules! Depcrate_binary_formatwrite_hashes {
() => {
// Module: crate::binary_format
// Provides: {"write_hashes"}
// Dependencies: {}
# [doc = " Writes a slice of clause hashes for a varisat proof"] fn write_hashes (target : & mut impl Write , hashes : & [ClauseHash]) -> io :: Result < () > { write_u64 (& mut * target , hashes . len () as u64) ? ; for & hash in hashes { write_u64 (& mut * target , hash as u64) ? ; } Ok (()) }
};
}
