// Generated macro for ZeroMapBorrowed (struct)
macro_rules! Depcrate_map_borrowedZeroMapBorrowed {
() => {
// Module: crate::map::borrowed
// Provides: {"ZeroMapBorrowed"}
// Dependencies: {}
# [doc = " A borrowed-only version of [`ZeroMap`](super::ZeroMap)"] # [doc = ""] # [doc = " This is useful for fully-zero-copy deserialization from non-human-readable"] # [doc = " serialization formats. It also has the advantage that it can return references that live for"] # [doc = " the lifetime of the backing buffer as opposed to that of the [`ZeroMapBorrowed`] instance."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::maps::ZeroMapBorrowed;"] # [doc = ""] # [doc = " // Example byte buffer representing the map { 1: \"one\" }"] # [doc = " let BINCODE_BYTES: &[u8; 25] = &["] # [doc = "     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 111,"] # [doc = "     110, 101,"] # [doc = " ];"] # [doc = ""] # [doc = " // Deserializing to ZeroMap requires no heap allocations."] # [doc = " let zero_map: ZeroMapBorrowed<u32, str> ="] # [doc = "     bincode::deserialize(BINCODE_BYTES)"] # [doc = "         .expect(\"Should deserialize successfully\");"] # [doc = " assert_eq!(zero_map.get(&1), Some(\"one\"));"] # [doc = " ```"] # [doc = ""] # [doc = " This can be obtained from a [`ZeroMap`](super::ZeroMap) via [`ZeroMap::as_borrowed`](super::ZeroMap::as_borrowed)"] pub struct ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K : ? Sized , V : ? Sized , { pub (crate) keys : & 'a < K as ZeroMapKV < 'a > > :: Slice , pub (crate) values : & 'a < V as ZeroMapKV < 'a > > :: Slice , }
};
}
