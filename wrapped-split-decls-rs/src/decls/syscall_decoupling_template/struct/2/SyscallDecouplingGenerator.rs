use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SyscallDecouplingGenerator { analysis_report : SyscallAnalysisReport , sparql_complexity_data : HashMap < String , f64 > , }