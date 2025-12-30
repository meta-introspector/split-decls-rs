// Generated macro for create_original_changelog_url (function)
macro_rules! Depcrate_publishcreate_original_changelog_url {
() => {
// Module: crate::publish
// Provides: {"create_original_changelog_url"}
// Dependencies: {}
fn create_original_changelog_url (file_name : & str) -> String { let year = & file_name [0 .. 4] ; let month = & file_name [5 .. 7] ; let day = & file_name [8 .. 10] ; let mut stem = & file_name [11 ..] ; if let Some (stripped) = stem . strip_suffix (".adoc") { stem = stripped ; } format ! ("https://rust-analyzer.github.io/thisweek/{year}/{month}/{day}/{stem}.html") }
};
}
