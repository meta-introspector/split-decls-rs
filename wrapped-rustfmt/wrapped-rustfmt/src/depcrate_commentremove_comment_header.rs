// Generated macro for remove_comment_header (function)
macro_rules! Depcrate_commentremove_comment_header {
() => {
// Module: crate::comment
// Provides: {"remove_comment_header"}
// Dependencies: {}
fn remove_comment_header (comment : & str) -> & str { if comment . starts_with ("///") || comment . starts_with ("//!") { & comment [3 ..] } else if let Some (stripped) = comment . strip_prefix ("//") { stripped } else if (comment . starts_with ("/**") && ! comment . starts_with ("/**/")) || comment . starts_with ("/*!") { & comment [3 .. comment . len () - 2] } else { assert ! (comment . starts_with ("/*") , "string '{comment}' is not a comment") ; & comment [2 .. comment . len () - 2] } }
};
}
