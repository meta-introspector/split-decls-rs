use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct TypeExtractorVisitor < 'a > { discovery : & 'a mut SynTypeDiscovery , }