use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Signature lookup table for fast access"] pub struct SignatureLookup { pub prime_to_signature : HashMap < u64 , String > , pub emoji_to_signature : HashMap < String , String > , pub signature_to_bindings : HashMap < String , Vec < String > > , }