// Generated macro for SHY (const)
macro_rules! Depcrate_word_separatorsSHY {
() => {
// Module: crate::word_separators
// Provides: {"SHY"}
// Dependencies: {}
# [doc = " Soft hyphen, also knows as a “shy hyphen”. Should show up as ‘-’"] # [doc = " if a line is broken at this point, and otherwise be invisible."] # [doc = " Textwrap does not currently support breaking words at soft"] # [doc = " hyphens."] # [cfg (feature = "unicode-linebreak")] const SHY : char = '\u{00ad}' ;
};
}
