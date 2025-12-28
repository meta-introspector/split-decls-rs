use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl TryFrom < & [u8] > for ObjectIdentifier { type Error = Error ; fn try_from (ber_bytes : & [u8]) -> Result < Self > { Self :: from_bytes (ber_bytes) } }