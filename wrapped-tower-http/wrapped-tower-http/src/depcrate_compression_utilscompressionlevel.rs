// Generated macro for CompressionLevel (enum)
macro_rules! Depcrate_compression_utilsCompressionLevel {
() => {
// Module: crate::compression_utils
// Provides: {"CompressionLevel"}
// Dependencies: {}
# [doc = " Level of compression data should be compressed with."] # [non_exhaustive] # [derive (Clone , Copy , Debug , Eq , PartialEq , Default)] pub enum CompressionLevel { # [doc = " Fastest quality of compression, usually produces bigger size."] Fastest , # [doc = " Best quality of compression, usually produces the smallest size."] Best , # [doc = " Default quality of compression defined by the selected compression"] # [doc = " algorithm."] # [default] Default , # [doc = " Precise quality based on the underlying compression algorithms'"] # [doc = " qualities."] # [doc = ""] # [doc = " The interpretation of this depends on the algorithm chosen and the"] # [doc = " specific implementation backing it."] # [doc = ""] # [doc = " Qualities are implicitly clamped to the algorithm's maximum."] Precise (i32) , }
};
}
