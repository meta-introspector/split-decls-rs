use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl TraitBoundModifiers { pub const NONE : Self = Self { constness : BoundConstness :: Never , asyncness : BoundAsyncness :: Normal , polarity : BoundPolarity :: Positive , } ; }