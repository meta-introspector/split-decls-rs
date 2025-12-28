use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [must_use] pub struct TimingGuard < 'a > (Option < measureme :: TimingGuard < 'a > >) ;