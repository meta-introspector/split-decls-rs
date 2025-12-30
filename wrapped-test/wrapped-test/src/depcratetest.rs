// Generated macro for Test (struct)
macro_rules! DepcrateTest {
() => {
// Module: crate
// Provides: {"Test"}
// Dependencies: {}
# [doc = " Helper structure representing a discovered `test.wit` file."] struct Test { # [doc = " The name of this test, unique amongst all tests."] # [doc = ""] # [doc = " Inferred from the directory name."] name : String , # [doc = " Path to the root of this test."] path : PathBuf , # [doc = " Configuration for this test, specified in the WIT file."] config : config :: WitConfig , kind : TestKind , }
};
}
