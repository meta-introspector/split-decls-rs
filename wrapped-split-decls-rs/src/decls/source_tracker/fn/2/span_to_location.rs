use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Extract source location from a span"] pub fn span_to_location (span : Span) -> SourceLocation { let line_col : LineColumn = span . start () ; SourceLocation { file : "unknown" . to_string () , line : line_col . line , column : line_col . column , } }