use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static mut TELEMETRY_LOG : Vec < TelemetryData > = Vec :: new () ;