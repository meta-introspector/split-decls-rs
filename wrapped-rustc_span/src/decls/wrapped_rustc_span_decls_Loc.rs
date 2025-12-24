use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A source code location used for error reporting.
#[derive(Debug, Clone)]
pub struct Loc {
    /// Information about the original source.
    pub file: Arc<SourceFile>,
    /// The (1-based) line number.
    pub line: usize,
    /// The (0-based) column offset.
    pub col: CharPos,
    /// The (0-based) column offset when displayed.
    pub col_display: usize,
}
