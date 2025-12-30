// Generated macro for impl_185 (impl)
macro_rules! Depcrate_readerimpl_185 {
() => {
// Module: crate::reader
// Provides: {"impl_185"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > ZeroTrieIterator < 'a > { pub (crate) fn new < S : AsRef < [u8] > + ? Sized > (store : & 'a S , use_phf : bool) -> Self { ZeroTrieIterator { use_phf , state : alloc :: vec ! [(store . as_ref () , alloc :: vec ! [] , 0)] , } } }
};
}
