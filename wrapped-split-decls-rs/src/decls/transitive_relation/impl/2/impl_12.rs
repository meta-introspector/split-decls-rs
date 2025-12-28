use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Eq + Hash > Default for TransitiveRelationBuilder < T > { fn default () -> Self { TransitiveRelationBuilder { elements : Default :: default () , edges : Default :: default () } } }