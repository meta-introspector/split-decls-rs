// Generated macro for macro_74 (macro)
macro_rules! Depcrate_builtins_filters_stringmacro_74 {
() => {
// Module: crate::builtins::filters::string
// Provides: {"macro_74"}
// Dependencies: {}
lazy_static ! { static ref STRIPTAGS_RE : Regex = Regex :: new (r"(<!--.*?-->|<[^>]*>)") . unwrap () ; static ref WORDS_RE : Regex = Regex :: new (r"\b(?P<first>[\w'])(?P<rest>[\w']*)\b") . unwrap () ; static ref SPACELESS_RE : Regex = Regex :: new (r">\s+<") . unwrap () ; }
};
}
