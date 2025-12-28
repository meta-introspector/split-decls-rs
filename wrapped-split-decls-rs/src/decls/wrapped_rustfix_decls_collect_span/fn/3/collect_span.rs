use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: collect_span");
# [doc = " Converts a [`DiagnosticSpan`] into a [`Replacement`]."] fn collect_span (span : & DiagnosticSpan) -> Option < Replacement > { let snippet = span_to_snippet (span) ; let replacement = span . suggested_replacement . clone () ? ; Some (Replacement { snippet , replacement , }) }
}