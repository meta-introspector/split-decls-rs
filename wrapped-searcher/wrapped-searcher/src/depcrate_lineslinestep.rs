// Generated macro for LineStep (struct)
macro_rules! Depcrate_linesLineStep {
() => {
// Module: crate::lines
// Provides: {"LineStep"}
// Dependencies: {}
# [doc = " An explicit iterator over lines in a particular slice of bytes."] # [doc = ""] # [doc = " This iterator avoids borrowing the bytes themselves, and instead requires"] # [doc = " callers to explicitly provide the bytes when moving through the iterator."] # [doc = " While not idiomatic, this provides a simple way of iterating over lines"] # [doc = " that doesn't require borrowing the slice itself, which can be convenient."] # [doc = ""] # [doc = " Line terminators are considered part of the line they terminate. All lines"] # [doc = " yielded by the iterator are guaranteed to be non-empty."] # [derive (Debug)] pub struct LineStep { line_term : u8 , pos : usize , end : usize , }
};
}
