use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl KeyValue { pub fn get_ident_for_err (& self) -> & Ident { match self { KeyValue :: Simple (key , _) => key , KeyValue :: Block (key , _) => key , KeyValue :: List (key , _) => key , KeyValue :: InlineTable (key , _) => key , } } }