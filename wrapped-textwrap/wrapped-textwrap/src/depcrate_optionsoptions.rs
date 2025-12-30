// Generated macro for Options (struct)
macro_rules! Depcrate_optionsOptions {
() => {
// Module: crate::options
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Holds configuration options for wrapping and filling text."] # [non_exhaustive] # [derive (Debug , Clone)] pub struct Options < 'a > { # [doc = " The width in columns at which the text will be wrapped."] pub width : usize , # [doc = " Line ending used for breaking lines."] pub line_ending : LineEnding , # [doc = " Indentation used for the first line of output. See the"] # [doc = " [`Options::initial_indent`] method."] pub initial_indent : & 'a str , # [doc = " Indentation used for subsequent lines of output. See the"] # [doc = " [`Options::subsequent_indent`] method."] pub subsequent_indent : & 'a str , # [doc = " Allow long words to be broken if they cannot fit on a line."] # [doc = " When set to `false`, some lines may be longer than"] # [doc = " `self.width`. See the [`Options::break_words`] method."] pub break_words : bool , # [doc = " Wrapping algorithm to use, see the implementations of the"] # [doc = " [`WrapAlgorithm`] trait for details."] pub wrap_algorithm : WrapAlgorithm , # [doc = " The line breaking algorithm to use, see the [`WordSeparator`]"] # [doc = " trait for an overview and possible implementations."] pub word_separator : WordSeparator , # [doc = " The method for splitting words. This can be used to prohibit"] # [doc = " splitting words on hyphens, or it can be used to implement"] # [doc = " language-aware machine hyphenation."] pub word_splitter : WordSplitter , # [doc = " Allow trailing spaces to be preserved at the end of the line."] pub preserve_trailing_space : bool , }
};
}
