use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T > Aligned for [T] { const ALIGN : Alignment = Alignment :: of :: < T > () ; }