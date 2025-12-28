use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Send > ExactSizeIterator for IntoIter < T > { }