use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn log_notify_error < T > (res : notify :: Result < T >) -> Option < T > { res . map_err (| err | tracing :: warn ! ("notify error: {}" , err)) . ok () }
}