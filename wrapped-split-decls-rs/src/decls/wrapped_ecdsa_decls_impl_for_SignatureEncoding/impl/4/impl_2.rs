use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " NOTE: this implementation assumes the default digest for the given elliptic"] # [doc = " curve as defined by [`hazmat::DigestAlgorithm`]."] # [doc = ""] # [doc = " When working with alternative digests, you will need to use e.g."] # [doc = " [`SignatureWithOid::new_with_digest`]."] # [cfg (all (feature = "digest" , feature = "hazmat"))] impl < C > SignatureEncoding for SignatureWithOid < C > where C : hazmat :: DigestAlgorithm , C :: Digest : AssociatedOid , SignatureSize < C > : ArraySize , { type Repr = SignatureBytes < C > ; }
}