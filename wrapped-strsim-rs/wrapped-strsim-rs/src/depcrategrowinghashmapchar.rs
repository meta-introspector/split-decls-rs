// Generated macro for GrowingHashmapChar (struct)
macro_rules! DepcrateGrowingHashmapChar {
() => {
// Module: crate
// Provides: {"GrowingHashmapChar"}
// Dependencies: {}
# [doc = " specialized hashmap to store user provided types"] # [doc = " this implementation relies on a couple of base assumptions in order to simplify the implementation"] # [doc = " - the hashmap does not have an upper limit of included items"] # [doc = " - the default value for the `ValueType` can be used as a dummy value to indicate an empty cell"] # [doc = " - elements can't be removed"] # [doc = " - only allocates memory on first write access."] # [doc = "   This improves performance for hashmaps that are never written to"] struct GrowingHashmapChar < ValueType > { used : i32 , fill : i32 , mask : i32 , map : Option < Vec < GrowingHashmapMapElemChar < ValueType > > > , }
};
}
