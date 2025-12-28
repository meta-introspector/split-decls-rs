use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "futures-01")] # [cfg_attr (docsrs , doc (cfg (feature = "futures-01")))] impl < T : futures_01 :: Stream > futures_01 :: Stream for Instrumented < T > { type Item = T :: Item ; type Error = T :: Error ; fn poll (& mut self) -> futures_01 :: Poll < Option < Self :: Item > , Self :: Error > { let _enter = self . span . enter () ; self . inner . poll () } }