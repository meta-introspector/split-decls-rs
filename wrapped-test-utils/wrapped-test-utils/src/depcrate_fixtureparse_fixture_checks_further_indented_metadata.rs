// Generated macro for parse_fixture_checks_further_indented_metadata (function)
macro_rules! Depcrate_fixtureparse_fixture_checks_further_indented_metadata {
() => {
// Module: crate::fixture
// Provides: {"parse_fixture_checks_further_indented_metadata"}
// Dependencies: {}
# [test] # [should_panic] fn parse_fixture_checks_further_indented_metadata () { FixtureWithProjectMeta :: parse (r"
        //- /lib.rs
          mod bar;

          fn foo() {}
          //- /bar.rs
          pub fn baz() {}
          " ,) ; }
};
}
