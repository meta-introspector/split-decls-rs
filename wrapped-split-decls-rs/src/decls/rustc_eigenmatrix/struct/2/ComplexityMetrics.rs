use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct ComplexityMetrics { pub cyclomatic : f64 , pub cognitive : f64 , pub halstead : f64 , pub maintainability : f64 , }