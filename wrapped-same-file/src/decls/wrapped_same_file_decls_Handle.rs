use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A handle to a file that can be tested for equality with other handles.
///
/// If two files are the same, then any two handles of those files will compare
/// equal. If two files are not the same, then any two handles of those files
/// will compare not-equal.
///
/// A handle consumes an open file resource as long as it exists.
///
/// Equality is determined by comparing inode numbers on Unix and a combination
/// of identifier, volume serial, and file size on Windows. Note that it's
/// possible for comparing two handles to produce a false positive on some
/// platforms. Namely, two handles can compare equal even if the two handles
/// *don't* point to the same file. Check the [source] for specific
/// implementation details.
///
/// [source]: https://github.com/BurntSushi/same-file/tree/master/src
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Handle(imp::Handle);
