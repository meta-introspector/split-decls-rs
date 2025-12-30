// Generated macro for LineBreakOptions (struct)
macro_rules! Depcrate_lineLineBreakOptions {
() => {
// Module: crate::line
// Provides: {"LineBreakOptions"}
// Dependencies: {}
# [doc = " Options to tailor line-breaking behavior."] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , Debug , Default)] pub struct LineBreakOptions < 'a > { # [doc = " Strictness of line-breaking rules. See [`LineBreakStrictness`]."] # [doc = ""] # [doc = " Default is [`LineBreakStrictness::Strict`]"] pub strictness : Option < LineBreakStrictness > , # [doc = " Line break opportunities between letters. See [`LineBreakWordOption`]."] # [doc = ""] # [doc = " Default is [`LineBreakStrictness::Normal`]"] pub word_option : Option < LineBreakWordOption > , # [doc = " Content locale for line segmenter"] # [doc = ""] # [doc = " This allows more break opportunities when `LineBreakStrictness` is"] # [doc = " `Normal` or `Loose`. See"] # [doc = " <https://drafts.csswg.org/css-text-3/#line-break-property> for details."] # [doc = " This option has no effect in Latin-1 mode."] pub content_locale : Option < & 'a LanguageIdentifier > , }
};
}
