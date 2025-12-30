// Generated macro for TestType (enum)
macro_rules! Depcrate_typesTestType {
() => {
// Module: crate::types
// Provides: {"TestType"}
// Dependencies: {}
# [doc = " Type of the test according to the [Rust book](https://doc.rust-lang.org/cargo/guide/tests.html)"] # [doc = " conventions."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum TestType { # [doc = " Unit-tests are expected to be in the `src` folder of the crate."] UnitTest , # [doc = " Integration-style tests are expected to be in the `tests` folder of the crate."] IntegrationTest , # [doc = " Doctests are created by the `librustdoc` manually, so it's a different type of test."] DocTest , # [doc = " Tests for the sources that don't follow the project layout convention"] # [doc = " (e.g. tests in raw `main.rs` compiled by calling `rustc --test` directly)."] Unknown , }
};
}
