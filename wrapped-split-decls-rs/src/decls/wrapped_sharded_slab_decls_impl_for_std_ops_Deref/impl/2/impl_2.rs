use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , C > std :: ops :: Deref for OwnedEntry < T , C > where C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }