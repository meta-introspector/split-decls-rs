use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " [`GenericHkdf`] variant which uses [`SimpleHmac`] for the underlying HMAC implementation."] pub type SimpleHkdf < H > = GenericHkdf < SimpleHmac < H > > ;