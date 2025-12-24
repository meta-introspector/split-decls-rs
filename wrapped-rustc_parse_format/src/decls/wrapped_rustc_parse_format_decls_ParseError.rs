use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct ParseError {
    pub description: String,
    pub note: Option<String>,
    pub label: String,
    pub span: Range<usize>,
    pub secondary_label: Option<(String, Range<usize>)>,
    pub suggestion: Suggestion,
}
