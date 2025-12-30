// Generated macro for Utf16CharIndices (struct)
macro_rules! Depcrate_indicesUtf16CharIndices {
() => {
// Module: crate::indices
// Provides: {"Utf16CharIndices"}
// Dependencies: {}
# [doc = " An iterator over the [`char`]s  and their positions."] # [derive (Clone , Debug)] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Utf16CharIndices < 'a > { front_offset : usize , iter : Utf16Chars < 'a > , }
};
}
