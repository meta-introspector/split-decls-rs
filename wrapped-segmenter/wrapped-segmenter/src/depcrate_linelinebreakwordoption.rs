// Generated macro for LineBreakWordOption (enum)
macro_rules! Depcrate_lineLineBreakWordOption {
() => {
// Module: crate::line
// Provides: {"LineBreakWordOption"}
// Dependencies: {}
# [doc = " An enum specifies the line break opportunities between letters. It can be"] # [doc = " passed as an argument when creating a line segmenter."] # [doc = ""] # [doc = " Each enum value has the same meaning with respect to the `word-break`"] # [doc = " property values in the CSS Text spec. See the details in"] # [doc = " <https://drafts.csswg.org/css-text-3/#word-break-property>"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , Debug , Default)] pub enum LineBreakWordOption { # [doc = " Words break according to their customary rules. See the details in"] # [doc = " <https://drafts.csswg.org/css-text-3/#valdef-word-break-normal>."] # [default] Normal , # [doc = " Breaking is allowed within \"words\"."] # [doc = " <https://drafts.csswg.org/css-text-3/#valdef-word-break-break-all>"] BreakAll , # [doc = " Breaking is forbidden within \"word\"."] # [doc = " <https://drafts.csswg.org/css-text-3/#valdef-word-break-keep-all>"] KeepAll , }
};
}
