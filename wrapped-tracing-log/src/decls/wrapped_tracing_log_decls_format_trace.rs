use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Format a log record as a trace event in the current span.
pub fn format_trace(record: &log::Record<'_>) -> io::Result<()> {
    dispatch_record(record);
    Ok(())
}
