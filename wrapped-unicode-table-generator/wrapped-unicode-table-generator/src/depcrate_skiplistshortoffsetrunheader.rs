// Generated macro for ShortOffsetRunHeader (struct)
macro_rules! Depcrate_skiplistShortOffsetRunHeader {
() => {
// Module: crate::skiplist
// Provides: {"ShortOffsetRunHeader"}
// Dependencies: {}
# [doc = " This will get packed into a single u32 before inserting into the data set."] # [derive (PartialEq)] struct ShortOffsetRunHeader { # [doc = " Note, we actually only allow for 11 bits here. This should be enough --"] # [doc = " our largest sets are around ~1400 offsets long."] start_index : u16 , # [doc = " Note, we only allow for 21 bits here."] prefix_sum : u32 , }
};
}
