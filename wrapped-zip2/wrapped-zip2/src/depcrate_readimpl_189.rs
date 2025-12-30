// Generated macro for impl_189 (impl)
macro_rules! Depcrate_readimpl_189 {
() => {
// Module: crate::read
// Provides: {"impl_189"}
// Dependencies: {}
impl < R : Seek > Seek for ZipFileSeek < '_ , R > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { match & mut self . reader { ZipFileSeekReader :: Raw (r) => r . seek (pos) , } } }
};
}
