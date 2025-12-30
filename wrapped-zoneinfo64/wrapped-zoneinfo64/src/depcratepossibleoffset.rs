// Generated macro for PossibleOffset (enum)
macro_rules! DepcratePossibleOffset {
() => {
// Module: crate
// Provides: {"PossibleOffset"}
// Dependencies: {}
# [doc = " Possible offsets for a local datetime"] # [derive (Debug , PartialEq)] pub enum PossibleOffset { # [doc = " There is a single possible offset"] Single (Offset) , # [doc = " There are multiple possible offsets, because we are inside a backward transition"] # [doc = ""] # [doc = " Note: Temporal requires these to be in ascending order of offset, Temporal consumers should sort them"] Ambiguous { # [doc = " The offset before the transition"] before : Offset , # [doc = " The offset after the transition"] after : Offset , # [doc = " The transition epoch in seconds"] transition : i64 , } , # [doc = " There is no possible offset, because we are at a forward transition"] None { # [doc = " The offset before this transition"] # [doc = ""] # [doc = " This is useful when performing fallback behavior on hitting a"] # [doc = " transition where the local time has a gap."] before : Offset , # [doc = " The offset after this transition"] after : Offset , # [doc = " The transition epoch in seconds"] transition : i64 , } , }
};
}
