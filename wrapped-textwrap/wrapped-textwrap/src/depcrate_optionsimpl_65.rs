// Generated macro for impl_65 (impl)
macro_rules! Depcrate_optionsimpl_65 {
() => {
// Module: crate::options
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > From < & 'a Options < 'a > > for Options < 'a > { fn from (options : & 'a Options < 'a >) -> Self { Self { width : options . width , line_ending : options . line_ending , initial_indent : options . initial_indent , subsequent_indent : options . subsequent_indent , break_words : options . break_words , word_separator : options . word_separator , wrap_algorithm : options . wrap_algorithm , word_splitter : options . word_splitter . clone () , preserve_trailing_space : options . preserve_trailing_space , } } }
};
}
