// Generated macro for UnicodeDataExpander (struct)
macro_rules! Depcrate_unicode_dataUnicodeDataExpander {
() => {
// Module: crate::unicode_data
// Provides: {"UnicodeDataExpander"}
// Dependencies: {}
# [doc = " An iterator adapter that expands rows in `UnicodeData.txt`."] # [doc = ""] # [doc = " Throughout `UnicodeData.txt`, some assigned codepoints are not explicitly"] # [doc = " represented. Instead, they are represented by a pair of rows, indicating"] # [doc = " a range of codepoints with the same properties. For example, the Hangul"] # [doc = " syllable codepoints are represented by these two rows:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " AC00;<Hangul Syllable, First>;Lo;0;L;;;;;N;;;;;"] # [doc = " D7A3;<Hangul Syllable, Last>;Lo;0;L;;;;;N;;;;;"] # [doc = " ```"] # [doc = ""] # [doc = " This iterator will wrap any iterator of `UnicodeData` and, when a range of"] # [doc = " Unicode codepoints is found, it will be expanded to the appropriate"] # [doc = " sequence of `UnicodeData` values. Note that all such expanded records will"] # [doc = " have an empty name."] pub struct UnicodeDataExpander < I : Iterator > { # [doc = " The underlying iterator."] it : std :: iter :: Peekable < I > , # [doc = " A range of codepoints to emit when we've found a pair. Otherwise,"] # [doc = " `None`."] range : CodepointRange , }
};
}
