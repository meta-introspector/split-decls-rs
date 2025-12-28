use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Test span_to_snippet for a span ending at the end of a `SourceFile`."] # [test] fn t8 () { let sm = init_source_map () ; let span = Span :: with_root_ctxt (BytePos (12) , BytePos (23)) ; let snippet = sm . span_to_snippet (span) ; assert_eq ! (snippet , Ok ("second line" . to_string ())) ; }
}