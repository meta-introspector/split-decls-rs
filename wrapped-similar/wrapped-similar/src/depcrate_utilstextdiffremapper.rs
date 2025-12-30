// Generated macro for TextDiffRemapper (struct)
macro_rules! Depcrate_utilsTextDiffRemapper {
() => {
// Module: crate::utils
// Provides: {"TextDiffRemapper"}
// Dependencies: {}
# [doc = " A remapper that can remap diff ops to the original slices."] # [doc = ""] # [doc = " The idea here is that when a [`TextDiff`](crate::TextDiff) is created from"] # [doc = " two strings and the internal tokenization is used, this remapper can take"] # [doc = " a range in the tokenized sequences and remap it to the original string."] # [doc = " This is particularly useful when you want to do things like character or"] # [doc = " grapheme level diffs but you want to not have to iterate over small sequences"] # [doc = " but large consequitive ones from the source."] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::{ChangeTag, TextDiff};"] # [doc = " use similar::utils::TextDiffRemapper;"] # [doc = ""] # [doc = " let old = \"yo! foo bar baz\";"] # [doc = " let new = \"yo! foo bor baz\";"] # [doc = " let diff = TextDiff::from_words(old, new);"] # [doc = " let remapper = TextDiffRemapper::from_text_diff(&diff, old, new);"] # [doc = " let changes: Vec<_> = diff.ops()"] # [doc = "     .iter()"] # [doc = "     .flat_map(move |x| remapper.iter_slices(x))"] # [doc = "     .collect();"] # [doc = ""] # [doc = " assert_eq!(changes, vec!["] # [doc = "     (ChangeTag::Equal, \"yo! foo \"),"] # [doc = "     (ChangeTag::Delete, \"bar\"),"] # [doc = "     (ChangeTag::Insert, \"bor\"),"] # [doc = "     (ChangeTag::Equal, \" baz\")"] # [doc = " ]);"] pub struct TextDiffRemapper < 'x , T : ? Sized > { old : SliceRemapper < 'x , T > , new : SliceRemapper < 'x , T > , }
};
}
