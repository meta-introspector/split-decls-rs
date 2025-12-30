// Generated macro for get_iter_phf (function)
macro_rules! Depcrate_readerget_iter_phf {
() => {
// Module: crate::reader
// Provides: {"get_iter_phf"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub (crate) fn get_iter_phf < S : AsRef < [u8] > + ? Sized > (store : & S) -> ZeroTrieIterator < '_ > { ZeroTrieIterator :: new (store , true) }
};
}
