// Generated macro for CompressionLevel (enum)
macro_rules! Depcrate_compressCompressionLevel {
() => {
// Module: crate::compress
// Provides: {"CompressionLevel"}
// Dependencies: {}
# [doc = " A hint for how many resources to dedicate to a compression."] # [non_exhaustive] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum CompressionLevel { # [doc = " This compression is happening interactively during a handshake."] # [doc = ""] # [doc = " Implementations may wish to choose a conservative compression level."] Interactive , # [doc = " The compression may be amortized over many connections."] # [doc = ""] # [doc = " Implementations may wish to choose an aggressive compression level."] Amortized , }
};
}
