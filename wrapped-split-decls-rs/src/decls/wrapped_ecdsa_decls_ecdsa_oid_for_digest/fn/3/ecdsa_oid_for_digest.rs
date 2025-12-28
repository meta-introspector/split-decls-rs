use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Get the ECDSA OID for a given digest OID."] # [cfg (feature = "digest")] const fn ecdsa_oid_for_digest (digest_oid : ObjectIdentifier) -> Option < ObjectIdentifier > { match digest_oid { SHA224_OID => Some (ECDSA_SHA224_OID) , SHA256_OID => Some (ECDSA_SHA256_OID) , SHA384_OID => Some (ECDSA_SHA384_OID) , SHA512_OID => Some (ECDSA_SHA512_OID) , _ => None , } }
}