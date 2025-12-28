use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " OID for ECDSA with SHA-384 digests."] # [doc = ""] # [doc = " ```text"] # [doc = " ecdsa-with-SHA384 OBJECT IDENTIFIER ::= { iso(1) member-body(2)"] # [doc = "      us(840) ansi-X9-62(10045) signatures(4) ecdsa-with-SHA2(3) 3 }"] # [doc = " ```"] # [cfg (feature = "digest")] pub const ECDSA_SHA384_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("1.2.840.10045.4.3.3") ;