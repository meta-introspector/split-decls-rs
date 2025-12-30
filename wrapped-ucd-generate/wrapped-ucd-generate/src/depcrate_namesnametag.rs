// Generated macro for NameTag (enum)
macro_rules! Depcrate_namesNameTag {
() => {
// Module: crate::names
// Provides: {"NameTag"}
// Dependencies: {}
# [doc = " A tag indicating how the name of a codepoint was found."] # [doc = ""] # [doc = " When a name has both an algorithmically generated name and an"] # [doc = " explicit/alias name, then the algorithmically generated tag is preferred."] # [derive (Debug)] enum NameTag { # [doc = " The name is listed explicitly in UnicodeData.txt."] Explicit , # [doc = " The name was taken from NameAliases.txt."] Alias , # [doc = " The name is an algorithmically generated Hangul syllable."] Hangul , # [doc = " The name is an algorithmically generated ideograph."] Ideograph , }
};
}
