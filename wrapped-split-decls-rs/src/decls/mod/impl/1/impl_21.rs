use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'g , N : Debug , E : Debug > ExactSizeIterator for DepthFirstTraversal < 'g , N , E > { }