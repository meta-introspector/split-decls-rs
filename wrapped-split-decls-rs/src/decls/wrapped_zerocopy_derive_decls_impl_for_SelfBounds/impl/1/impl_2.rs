use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [allow (clippy :: needless_lifetimes)] impl < 'a > SelfBounds < 'a > { const SIZED : Self = Self :: All (& [Trait :: Sized]) ; }