// Generated macro for tests (module)
macro_rules! Depcrate_publishtests {
() => {
// Module: crate::publish
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn original_changelog_url_creation () { let input = "2019-07-24-changelog-0.adoc" ; let actual = create_original_changelog_url (input) ; let expected = "https://rust-analyzer.github.io/thisweek/2019/07/24/changelog-0.html" ; assert_eq ! (actual , expected) ; } }
};
}
