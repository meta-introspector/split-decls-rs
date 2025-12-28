use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Cancellable < T > = Result < T , Cancelled > ;