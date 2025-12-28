use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Test `span_to_lines` for a span ending at the end of a `SourceFile`."] # [test] fn t7 () { let sm = init_source_map () ; let span = Span :: with_root_ctxt (BytePos (12) , BytePos (23)) ; let file_lines = sm . span_to_lines (span) . unwrap () ; assert_eq ! (file_lines . file . name , PathBuf :: from ("blork.rs") . into ()) ; assert_eq ! (file_lines . lines . len () , 1) ; assert_eq ! (file_lines . lines [0] . line_index , 1) ; }
}