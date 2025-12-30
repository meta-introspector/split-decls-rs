// Generated macro for ZeroTriePerfectHash (struct)
macro_rules! Depcrate_zerotrieZeroTriePerfectHash {
() => {
// Module: crate::zerotrie
// Provides: {"ZeroTriePerfectHash"}
// Dependencies: {}
# [doc = " A data structure that compactly maps from byte strings to integers."] # [doc = ""] # [doc = " For more information, see [`ZeroTrie`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use litemap::LiteMap;"] # [doc = " use zerotrie::ZeroTriePerfectHash;"] # [doc = ""] # [doc = " let mut map = LiteMap::<&[u8], usize>::new_vec();"] # [doc = " map.insert(\"foo\".as_bytes(), 1);"] # [doc = " map.insert(\"bår\".as_bytes(), 2);"] # [doc = " map.insert(\"båzzøø\".as_bytes(), 3);"] # [doc = ""] # [doc = " let trie = ZeroTriePerfectHash::try_from(&map)?;"] # [doc = ""] # [doc = " assert_eq!(trie.get(\"foo\".as_bytes()), Some(1));"] # [doc = " assert_eq!(trie.get(\"bår\".as_bytes()), Some(2));"] # [doc = " assert_eq!(trie.get(\"båzzøø\".as_bytes()), Some(3));"] # [doc = " assert_eq!(trie.get(\"bazzoo\".as_bytes()), None);"] # [doc = ""] # [doc = " # Ok::<_, zerotrie::ZeroTrieBuildError>(())"] # [doc = " ```"] # [repr (transparent)] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq)] # [cfg_attr (feature = "databake" , derive (databake :: Bake))] # [cfg_attr (feature = "databake" , databake (path = zerotrie))] # [allow (clippy :: exhaustive_structs)] pub struct ZeroTriePerfectHash < Store : ? Sized > { # [doc (hidden)] pub store : Store , }
};
}
