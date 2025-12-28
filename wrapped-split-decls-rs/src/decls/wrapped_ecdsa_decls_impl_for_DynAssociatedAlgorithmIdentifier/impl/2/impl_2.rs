use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > DynAssociatedAlgorithmIdentifier for SignatureWithOid < C > where C : EcdsaCurve , { fn algorithm_identifier (& self) -> spki :: Result < AlgorithmIdentifierOwned > { Ok (AlgorithmIdentifierOwned { oid : self . oid , parameters : None , }) } }