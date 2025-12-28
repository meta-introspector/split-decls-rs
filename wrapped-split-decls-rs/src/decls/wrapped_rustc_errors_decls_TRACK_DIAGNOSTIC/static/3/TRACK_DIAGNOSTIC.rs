use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
# [doc = " Diagnostics emitted by `DiagCtxtInner::emit_diagnostic` are passed through this function. Used"] # [doc = " for tracking by incremental, to replay diagnostics as necessary."] pub static TRACK_DIAGNOSTIC : AtomicRef < fn (DiagInner , & mut dyn FnMut (DiagInner) -> Option < ErrorGuaranteed >) -> Option < ErrorGuaranteed > , > = AtomicRef :: new (& (default_track_diagnostic as _)) ;
}