use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: deserialize_request");
# [doc = " Deserialize a request from Cargo."] fn deserialize_request (value : & str ,) -> Result < CredentialRequest < '_ > , Box < dyn std :: error :: Error + Send + Sync > > { let request : CredentialRequest < '_ > = serde_json :: from_str (& value) ? ; if request . v != PROTOCOL_VERSION_1 { return Err (format ! ("unsupported protocol version {}" , request . v) . into ()) ; } Ok (request) }
}