use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct ConditionalWrapperInput { feature : LitStr , original : syn :: Path , wrapper : syn :: Path , }