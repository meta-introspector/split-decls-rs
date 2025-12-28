use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct LocalSource { pub local : Local , pub source : InFile < Either < ast :: IdentPat , ast :: SelfParam > > , }