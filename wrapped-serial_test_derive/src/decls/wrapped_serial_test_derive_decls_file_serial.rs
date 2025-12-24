use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Allows for the creation of file-serialised Rust tests
/// ````no_run
/// #[test]
/// #[file_serial]
/// fn test_serial_one() {
///   // Do things
/// }
///
/// #[test]
/// #[file_serial]
/// fn test_serial_another() {
///   // Do things
/// }
/// ````
///
/// Multiple tests with the [file_serial](macro@file_serial) attribute are guaranteed to run in serial, as per the [serial](macro@serial)
/// attribute. Note that there are no guarantees about one test with [serial](macro@serial) and another with [file_serial](macro@file_serial)
/// as they lock using different methods, and [file_serial](macro@file_serial) does not support nested serialised tests, but otherwise acts
/// like [serial](macro@serial). If you have other tests that can be run in parallel, but would clash
/// if run at the same time as the [file_serial](macro@file_serial) tests, you can use the [file_parallel](macro@file_parallel) attribute.
///
/// It also supports an optional `path` arg as well as key(s) as per [serial](macro@serial), which is the path to the file used for
/// locking purposes. This file is managed by `serial_test` and no assumptions about it's format should be made. The `path` defaults to
/// a file under a reasonable temp directory for the OS if not specified. If the `path` is specified, you can only use one key, as we
/// can't generate per-key paths if you've done that.
/// ````no_run
/// #[test]
/// #[file_serial(key)]
/// fn test_serial_one() {
///   // Do things
/// }
///
/// #[test]
/// #[file_serial(key, path => "/tmp/foo")]
/// fn test_serial_another() {
///   // Do things
/// }
/// ````
#[proc_macro_attribute]
#[cfg_attr(docsrs, doc(cfg(feature = "file_locks")))]
pub fn file_serial(attr: TokenStream, input: TokenStream) -> TokenStream {
    fs_serial_core(attr.into(), input.into()).into()
}
