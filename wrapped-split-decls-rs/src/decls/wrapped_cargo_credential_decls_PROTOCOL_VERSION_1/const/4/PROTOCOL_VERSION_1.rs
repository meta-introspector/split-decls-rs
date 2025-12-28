use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Credential process JSON protocol version."] # [doc = ""] # [doc = " If the protocol needs to make"] # [doc = " a breaking change, a new protocol version should be defined (`PROTOCOL_VERSION_2`)."] # [doc = " This library should offer support for both protocols if possible, by signaling"] # [doc = " in the `CredentialHello` message. Cargo will then choose which protocol to use,"] # [doc = " or it will error if there are no common protocol versions available."] pub const PROTOCOL_VERSION_1 : u32 = 1 ;
}