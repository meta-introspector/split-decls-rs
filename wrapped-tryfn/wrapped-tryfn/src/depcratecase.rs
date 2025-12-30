// Generated macro for Case (struct)
macro_rules! DepcrateCase {
() => {
// Module: crate
// Provides: {"Case"}
// Dependencies: {}
# [doc = " A test case enumerated by the [`Harness`] with data from the [`Setup`] function"] pub struct Case { # [doc = " Display name"] pub name : String , # [doc = " Input for the test"] pub fixture : std :: path :: PathBuf , # [doc = " What the actual output should be compared against or updated"] # [doc = ""] # [doc = " Generally derived from `fixture` and loaded with [`Data::read_from`]"] pub expected : Data , }
};
}
