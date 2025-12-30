// Generated macro for impl_155 (impl)
macro_rules! Depcrate_readimpl_155 {
() => {
// Module: crate::read
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a , R : Read > ZipFileReader < 'a , R > { fn into_inner (self) -> io :: Result < io :: Take < & 'a mut R > > { match self { ZipFileReader :: NoReader => invalid_state () , ZipFileReader :: Raw (r) => Ok (r) , ZipFileReader :: Compressed (r) => { Ok (r . into_inner () . into_inner () ? . into_inner () . into_inner ()) } } } }
};
}
