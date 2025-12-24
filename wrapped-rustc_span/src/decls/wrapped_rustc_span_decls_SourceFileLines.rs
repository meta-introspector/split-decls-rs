use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone)]
pub enum SourceFileLines {
    /// The source file lines, in decoded (random-access) form.
    Lines(Vec<RelativeBytePos>),
    /// The source file lines, in undecoded difference list form.
    Diffs(SourceFileDiffs),
}
