use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Algorithm [`ObjectIdentifier`] for elliptic curve public key cryptography (`id-ecPublicKey`).
///
/// <http://oid-info.com/get/1.2.840.10045.2.1>
#[cfg(feature = "der")]
pub const ALGORITHM_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap(
    "1.2.840.10045.2.1",
);
