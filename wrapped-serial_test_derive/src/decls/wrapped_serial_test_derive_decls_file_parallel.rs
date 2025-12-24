use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Allows for the creation of file-serialised parallel Rust tests that won't clash with file-serialised serial tests
/// ````no_run
/// #[test]
/// #[file_serial]
/// fn test_serial_one() {
///   // Do things
/// }
///
/// #[test]
/// #[file_parallel]
/// fn test_parallel_one() {
///   // Do things
/// }
///
/// #[test]
/// #[file_parallel]
/// fn test_parallel_two() {
///   // Do things
/// }
/// ````
/// Effectively, this should behave like [parallel](macro@parallel) but for [file_serial](macro@file_serial).
/// Note that as per [file_serial](macro@file_serial) this doesn't do anything for [serial](macro@serial)/[parallel](macro@parallel) tests.
///
/// It also supports an optional `path` arg as well as key(s) as per [serial](macro@serial), which is the path to the file used for
/// locking purposes. This file is managed by `serial_test` and no assumptions about it's format should be made. The `path` defaults to
/// a file under a reasonable temp directory for the OS if not specified. If the `path` is specified, you can only use one key, as we
/// can't generate per-key paths if you've done that.
/// ````no_run
/// #[test]
/// #[file_parallel(key, path => "/tmp/foo")]
/// fn test_parallel_one() {
///   // Do things
/// }
///
/// #[test]
/// #[file_parallel(key, path => "/tmp/foo")]
/// fn test_parallel_another() {
///   // Do things
/// }
/// ````
#[proc_macro_attribute]
#[cfg_attr(docsrs, doc(cfg(feature = "file_locks")))]
pub fn file_parallel(attr: TokenStream, input: TokenStream) -> TokenStream {
    fs_parallel_core(attr.into(), input.into()).into()
}
