// Generated macro for hash_one_file (function)
macro_rules! Depcratehash_one_file {
() => {
// Module: crate
// Provides: {"hash_one_file"}
// Dependencies: {}
# [inline (never)] fn hash_one_file (config : & Config , path : & Path , buffer : & mut [u8]) -> Result < u64 > { let mut file = File :: open (path) ? ; let mut hasher = XxHash3_64 :: with_seed (0) ; let (tx_empty , rx_empty) = mpsc :: channel () ; let (tx_filled , rx_filled) = mpsc :: channel () ; for buffer in buffer . chunks_mut (config . buffer_size) { tx_empty . send (buffer) . expect ("Must be able to populate initial buffers") ; } thread :: scope (| scope | { let thread_reader = scope . spawn (move | | { while let Ok (buffer) = rx_empty . recv () { let n_bytes = file . read (buffer) ? ; if n_bytes == 0 { break ; } tx_filled . send ((buffer , n_bytes)) . map_err (| _ | SendError (())) ? ; } Ok :: < _ , Error > (()) }) ; let hasher = & mut hasher ; let thread_hasher = scope . spawn (move | | { while let Ok ((buffer , n_bytes)) = rx_filled . recv () { let valid = & buffer [.. n_bytes] ; hasher . write (valid) ; if tx_empty . send (buffer) . is_err () { continue ; } } Ok :: < _ , Error > (()) }) ; thread_reader . join () . unwrap () ? ; thread_hasher . join () . unwrap () ? ; Ok :: < _ , Error > (()) }) ? ; let hash = hasher . finish () ; Ok (hash) }
};
}
