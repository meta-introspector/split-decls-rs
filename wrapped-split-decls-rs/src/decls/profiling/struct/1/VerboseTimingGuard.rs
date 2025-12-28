use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [must_use] pub struct VerboseTimingGuard < 'a > { info : Option < VerboseInfo > , _guard : TimingGuard < 'a > , }