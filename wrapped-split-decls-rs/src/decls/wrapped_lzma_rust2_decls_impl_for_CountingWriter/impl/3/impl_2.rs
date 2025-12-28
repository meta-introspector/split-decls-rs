use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "encoder")] impl < W > CountingWriter < W > { fn new (inner : W) -> Self { Self { inner , bytes_written : 0 , } } fn bytes_written (& self) -> u64 { self . bytes_written } fn into_inner (self) -> W { self . inner } fn inner (& self) -> & W { & self . inner } fn inner_mut (& mut self) -> & mut W { & mut self . inner } }
}