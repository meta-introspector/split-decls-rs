use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T > Aligned for [T] { const ALIGN : Alignment = Alignment :: of :: < T > () ; }
}