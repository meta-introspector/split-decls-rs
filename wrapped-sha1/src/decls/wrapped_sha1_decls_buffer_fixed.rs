use serde::{Deserialize, Serialize};
use std::collections::HashMap;
digest::buffer_fixed!(
    #[doc = " SHA-1 hasher."] pub struct Sha1(block_api::Sha1Core); oid :
    "1.3.14.3.2.26"; impl : FixedHashTraits;
);
