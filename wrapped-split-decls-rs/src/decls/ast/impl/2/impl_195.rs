use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl BoundAsyncness { pub fn as_str (self) -> & 'static str { match self { Self :: Normal => "" , Self :: Async (_) => "async" , } } }