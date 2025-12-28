macro_rules! TrieSetSlice {
    () => {
        # [doc = " A borrowed trie set."] # [derive (Clone , Copy)] pub struct TrieSetSlice < 'a > { # [doc = " first tree, one level"] # [doc (hidden)] pub tree1_level1 : & 'a [u64] , # [doc = " second tree, first level"] # [doc (hidden)] pub tree2_level1 : & 'a [u8] , # [doc = " second tree, second level"] # [doc (hidden)] pub tree2_level2 : & 'a [u64] , # [doc = " third tree, first level"] # [doc (hidden)] pub tree3_level1 : & 'a [u8] , # [doc = " third tree, second level"] # [doc (hidden)] pub tree3_level2 : & 'a [u8] , # [doc = " third tree, third level"] # [doc (hidden)] pub tree3_level3 : & 'a [u64] , }
    };
}

TrieSetSlice!();