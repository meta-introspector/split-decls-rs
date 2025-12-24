use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn loglevel_to_cs(
    level: log::Level,
) -> (&'static dyn Callsite, &'static Fields, &'static Metadata<'static>) {
    match level {
        log::Level::Trace => (&TRACE_CS, &*TRACE_FIELDS, &TRACE_META),
        log::Level::Debug => (&DEBUG_CS, &*DEBUG_FIELDS, &DEBUG_META),
        log::Level::Info => (&INFO_CS, &*INFO_FIELDS, &INFO_META),
        log::Level::Warn => (&WARN_CS, &*WARN_FIELDS, &WARN_META),
        log::Level::Error => (&ERROR_CS, &*ERROR_FIELDS, &ERROR_META),
    }
}
