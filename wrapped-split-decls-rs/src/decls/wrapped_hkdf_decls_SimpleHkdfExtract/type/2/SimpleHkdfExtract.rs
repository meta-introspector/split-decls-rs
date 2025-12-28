use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " [`GenericHkdfExtract`] variant which uses [`SimpleHmac`] for the underlying HMAC implementation."] pub type SimpleHkdfExtract < H > = GenericHkdfExtract < SimpleHmac < H > > ;
}