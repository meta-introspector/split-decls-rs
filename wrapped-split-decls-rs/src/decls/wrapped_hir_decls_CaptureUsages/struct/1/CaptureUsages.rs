use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct CaptureUsages { parent : DefWithBodyId , spans : SmallVec < [mir :: MirSpan ; 3] > , }