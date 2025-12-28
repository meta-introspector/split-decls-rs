use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct DelayedDiagInner { inner : DiagInner , note : Backtrace , }