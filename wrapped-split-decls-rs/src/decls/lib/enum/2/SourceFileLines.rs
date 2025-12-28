use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone)] pub enum SourceFileLines { # [doc = " The source file lines, in decoded (random-access) form."] Lines (Vec < RelativeBytePos >) , # [doc = " The source file lines, in undecoded difference list form."] Diffs (SourceFileDiffs) , }
}