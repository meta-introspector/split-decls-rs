use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static mut TELEMETRY_LOG : Vec < TelemetryData > = Vec :: new () ;
}