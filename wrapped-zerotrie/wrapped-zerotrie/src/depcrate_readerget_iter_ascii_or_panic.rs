// Generated macro for get_iter_ascii_or_panic (function)
macro_rules! Depcrate_readerget_iter_ascii_or_panic {
() => {
// Module: crate::reader
// Provides: {"get_iter_ascii_or_panic"}
// Dependencies: {}
# [doc = " # Panics"] # [doc = " Panics if the trie contains non-ASCII items."] # [cfg (feature = "alloc")] # [expect (clippy :: type_complexity)] pub (crate) fn get_iter_ascii_or_panic < S : AsRef < [u8] > + ? Sized > (store : & S ,) -> core :: iter :: Map < ZeroTrieIterator < '_ > , fn ((Vec < u8 > , usize)) -> (String , usize) > { ZeroTrieIterator :: new (store , false) . map (| (k , v) | { # [expect (clippy :: unwrap_used)] let ascii_str = String :: from_utf8 (k) . unwrap () ; (ascii_str , v) }) }
};
}
