use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Bootstrap4: Telemetry-wrapped execution of extracted functions"] pub struct Bootstrap4Executor { audit_log : Vec < String > , }
}