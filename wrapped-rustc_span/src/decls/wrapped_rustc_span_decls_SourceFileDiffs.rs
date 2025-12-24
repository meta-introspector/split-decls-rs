use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The source file lines in difference list form. This matches the form
/// used within metadata, which saves space by exploiting the fact that the
/// lines list is sorted and individual lines are usually not that long.
///
/// We read it directly from metadata and only decode it into `Lines` form
/// when necessary. This is a significant performance win, especially for
/// small crates where very little of `std`'s metadata is used.
#[derive(Clone)]
pub struct SourceFileDiffs {
    /// Always 1, 2, or 4. Always as small as possible, while being big
    /// enough to hold the length of the longest line in the source file.
    /// The 1 case is by far the most common.
    bytes_per_diff: usize,
    /// The number of diffs encoded in `raw_diffs`. Always one less than
    /// the number of lines in the source file.
    num_diffs: usize,
    /// The diffs in "raw" form. Each segment of `bytes_per_diff` length
    /// encodes one little-endian diff. Note that they aren't LEB128
    /// encoded. This makes for much faster decoding. Besides, the
    /// bytes_per_diff==1 case is by far the most common, and LEB128
    /// encoding has no effect on that case.
    raw_diffs: Vec<u8>,
}
