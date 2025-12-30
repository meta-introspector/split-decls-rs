// Generated macro for Fragment (trait)
macro_rules! Depcrate_coreFragment {
() => {
// Module: crate::core
// Provides: {"Fragment"}
// Dependencies: {}
# [doc = " A (text) fragment denotes the unit which we wrap into lines."] # [doc = ""] # [doc = " Fragments represent an abstract _word_ plus the _whitespace_"] # [doc = " following the word. In case the word falls at the end of the line,"] # [doc = " the whitespace is dropped and a so-called _penalty_ is inserted"] # [doc = " instead (typically `\"-\"` if the word was hyphenated)."] # [doc = ""] # [doc = " For wrapping purposes, the precise content of the word, the"] # [doc = " whitespace, and the penalty is irrelevant. All we need to know is"] # [doc = " the displayed width of each part, which this trait provides."] pub trait Fragment : std :: fmt :: Debug { # [doc = " Displayed width of word represented by this fragment."] fn width (& self) -> f64 ; # [doc = " Displayed width of the whitespace that must follow the word"] # [doc = " when the word is not at the end of a line."] fn whitespace_width (& self) -> f64 ; # [doc = " Displayed width of the penalty that must be inserted if the"] # [doc = " word falls at the end of a line."] fn penalty_width (& self) -> f64 ; }
};
}
