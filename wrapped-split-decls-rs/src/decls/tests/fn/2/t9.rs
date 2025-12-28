use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Test `span_to_str` for a span ending at the end of a `SourceFile`."] # [test] fn t9 () { let sm = init_source_map () ; let span = Span :: with_root_ctxt (BytePos (12) , BytePos (23)) ; let sstr = sm . span_to_diagnostic_string (span) ; assert_eq ! (sstr , "blork.rs:2:1: 2:12") ; }