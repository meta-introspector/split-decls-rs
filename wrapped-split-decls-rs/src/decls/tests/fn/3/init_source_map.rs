use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn init_source_map () -> SourceMap { let sm = SourceMap :: new (FilePathMapping :: empty ()) ; sm . new_source_file (PathBuf :: from ("blork.rs") . into () , "first line.\nsecond line" . to_string ()) ; sm . new_source_file (PathBuf :: from ("empty.rs") . into () , String :: new ()) ; sm . new_source_file (PathBuf :: from ("blork2.rs") . into () , "first line.\nsecond line" . to_string ()) ; sm }
}