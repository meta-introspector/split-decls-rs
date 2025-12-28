use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "digest")] const SHA256_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("2.16.840.1.101.3.4.2.1") ;