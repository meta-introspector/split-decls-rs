use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct SynMold { pub signatures : Vec < SynSignature > , pub complexity_metrics : ComplexityMetrics , pub usage_patterns : HashMap < String , UsagePattern > , }