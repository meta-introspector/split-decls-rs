// Generated macro for try_read_all (function)
macro_rules! Depcrate_archivetry_read_all {
() => {
// Module: crate::archive
// Provides: {"try_read_all"}
// Dependencies: {}
# [doc = " Try to fill the buffer from the reader."] # [doc = ""] # [doc = " If the reader reaches its end before filling the buffer at all, returns `false`."] # [doc = " Otherwise returns `true`."] fn try_read_all < R : Read > (r : & mut R , buf : & mut [u8]) -> io :: Result < bool > { let mut read = 0 ; while read < buf . len () { match r . read (& mut buf [read ..]) ? { 0 => { if read == 0 { return Ok (false) ; } return Err (other ("failed to read entire block")) ; } n => read += n , } } Ok (true) }
};
}
