use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl NetworkOracle for DefaultNetworkOracle { fn audit_connect () -> Result < () , String > { eprintln ! ("NET_AUDIT: Connect operation") ; Ok (()) } fn check_endpoint_safety (endpoint : & str) -> bool { ! endpoint . contains ("localhost") || endpoint . starts_with ("https://") } }
}