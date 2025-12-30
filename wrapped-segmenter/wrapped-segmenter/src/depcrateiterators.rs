// Generated macro for iterators (module)
macro_rules! Depcrateiterators {
() => {
// Module: crate
// Provides: {"iterators"}
// Dependencies: {}
# [doc = " Types supporting iteration over segments. Obtained from the segmenter types."] pub mod iterators { pub use crate :: grapheme :: GraphemeClusterBreakIterator ; pub use crate :: line :: LineBreakIterator ; pub use crate :: sentence :: SentenceBreakIterator ; pub use crate :: word :: { WordBreakIterator , WordBreakIteratorWithWordType } ; }
};
}
