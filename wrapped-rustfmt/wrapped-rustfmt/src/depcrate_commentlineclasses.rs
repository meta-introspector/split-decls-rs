// Generated macro for LineClasses (struct)
macro_rules! Depcrate_commentLineClasses {
() => {
// Module: crate::comment
// Provides: {"LineClasses"}
// Dependencies: {}
# [doc = " An iterator over the lines of a string, paired with the char kind at the"] # [doc = " end of the line."] pub (crate) struct LineClasses < 'a > { base : iter :: Peekable < CharClasses < std :: str :: Chars < 'a > > > , kind : FullCodeCharKind , }
};
}
