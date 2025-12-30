// Generated macro for CollationData (struct)
macro_rules! Depcrate_collator_collator_serdeCollationData {
() => {
// Module: crate::collator::collator_serde
// Provides: {"CollationData"}
// Dependencies: {}
# [doc = " Serde counterpart for `CollationData`."] # [derive (serde :: Deserialize)] pub (crate) struct CollationData { pub (crate) trie : CodePointTrieToml , pub (crate) contexts : Vec < u16 > , pub (crate) ce32s : Vec < u32 > , pub (crate) ces : Vec < i64 > , }
};
}
