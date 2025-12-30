// Generated macro for LineIter (struct)
macro_rules! Depcrate_linesLineIter {
() => {
// Module: crate::lines
// Provides: {"LineIter"}
// Dependencies: {}
# [doc = " An iterator over lines in a particular slice of bytes."] # [doc = ""] # [doc = " Line terminators are considered part of the line they terminate. All lines"] # [doc = " yielded by the iterator are guaranteed to be non-empty."] # [doc = ""] # [doc = " `'b` refers to the lifetime of the underlying bytes."] # [derive (Debug)] pub struct LineIter < 'b > { bytes : & 'b [u8] , stepper : LineStep , }
};
}
