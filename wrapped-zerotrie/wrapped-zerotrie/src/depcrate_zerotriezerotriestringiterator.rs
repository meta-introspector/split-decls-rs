// Generated macro for ZeroTrieStringIterator (type)
macro_rules! Depcrate_zerotrieZeroTrieStringIterator {
() => {
// Module: crate::zerotrie
// Provides: {"ZeroTrieStringIterator"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "alloc")] pub type ZeroTrieStringIterator < 'a > = core :: iter :: Map < reader :: ZeroTrieIterator < 'a > , fn ((Vec < u8 > , usize)) -> (String , usize) > ;
};
}
