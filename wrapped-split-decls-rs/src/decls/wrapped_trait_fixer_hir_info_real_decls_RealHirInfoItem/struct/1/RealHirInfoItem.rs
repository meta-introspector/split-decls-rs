use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RealHirInfoItem < 'tcx > (pub Item < 'tcx >) ;