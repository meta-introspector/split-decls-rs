// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " Convenience function that reads all bytes from `reader` and deserializes"] # [doc = " them with `from_bytes`."] pub fn from_reader < T , R > (mut reader : R) -> Result < T , Error > where T : de :: DeserializeOwned , R : Read , { let mut buf = vec ! [] ; reader . read_to_end (& mut buf) . map_err (| e | { de :: Error :: custom (format_args ! ("could not read input: {}" , e)) }) ? ; from_bytes (& buf) }
};
}
