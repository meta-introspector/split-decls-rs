use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " [`GenericHkdfExtract`] variant which uses [`Hmac`] for the underlying HMAC implementation."] pub type HkdfExtract < H > = GenericHkdfExtract < Hmac < H > > ;