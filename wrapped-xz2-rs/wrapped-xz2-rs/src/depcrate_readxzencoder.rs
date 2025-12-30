// Generated macro for XzEncoder (struct)
macro_rules! Depcrate_readXzEncoder {
() => {
// Module: crate::read
// Provides: {"XzEncoder"}
// Dependencies: {}
# [doc = " A compression stream which wraps an uncompressed stream of data. Compressed"] # [doc = " data will be read from the stream."] pub struct XzEncoder < R : Read > { inner : bufread :: XzEncoder < BufReader < R > > , }
};
}
