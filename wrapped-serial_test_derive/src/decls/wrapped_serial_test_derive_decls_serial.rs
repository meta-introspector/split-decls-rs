use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Allows for the creation of serialised Rust tests
/// ````no_run
/// #[test]
/// #[serial]
/// fn test_serial_one() {
///   // Do things
/// }
///
/// #[test]
/// #[serial]
/// fn test_serial_another() {
///   // Do things
/// }
/// ````
/// Multiple tests with the [serial](macro@serial) attribute are guaranteed to be executed in serial. Ordering
/// of the tests is not guaranteed however. If you have other tests that can be run in parallel, but would clash
/// if run at the same time as the [serial](macro@serial) tests, you can use the [parallel](macro@parallel) attribute.
///
/// If you want different subsets of tests to be serialised with each
/// other, but not depend on other subsets, you can add a key argument to [serial](macro@serial), and all calls
/// with identical arguments will be called in serial. Multiple comma-separated keys will make a test run in serial with all of the sets with any of those keys.
///
/// ````no_run
/// #[test]
/// #[serial(something)]
/// fn test_serial_one() {
///   // Do things
/// }
///
/// #[test]
/// #[serial(something)]
/// fn test_serial_another() {
///   // Do things
/// }
///
/// #[test]
/// #[serial(other)]
/// fn test_serial_third() {
///   // Do things
/// }
///
/// #[test]
/// #[serial(other)]
/// fn test_serial_fourth() {
///   // Do things
/// }
///
/// #[test]
/// #[serial(something, other)]
/// fn test_serial_fifth() {
///   // Do things, eventually
/// }
/// ````
/// `test_serial_one` and `test_serial_another` will be executed in serial, as will `test_serial_third` and `test_serial_fourth`
/// but neither sequence will be blocked by the other. `test_serial_fifth` is blocked by tests in either sequence.
///
/// Nested serialised tests (i.e. a [serial](macro@serial) tagged test calling another) are supported.
#[proc_macro_attribute]
pub fn serial(attr: TokenStream, input: TokenStream) -> TokenStream {
    local_serial_core(attr.into(), input.into()).into()
}
