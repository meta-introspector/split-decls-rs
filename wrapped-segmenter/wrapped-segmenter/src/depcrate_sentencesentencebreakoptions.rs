// Generated macro for SentenceBreakOptions (struct)
macro_rules! Depcrate_sentenceSentenceBreakOptions {
() => {
// Module: crate::sentence
// Provides: {"SentenceBreakOptions"}
// Dependencies: {}
# [doc = " Options to tailor sentence breaking behavior."] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , Debug , Default)] pub struct SentenceBreakOptions < 'a > { # [doc = " Content locale for sentence segmenter."] # [doc = ""] # [doc = " If you know the language of the text being segmented, provide it here in order to produce"] # [doc = " higher quality breakpoints."] pub content_locale : Option < & 'a LanguageIdentifier > , # [doc = " Options independent of the locale"] pub invariant_options : SentenceBreakInvariantOptions , }
};
}
