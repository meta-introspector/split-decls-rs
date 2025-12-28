use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Eq , HashStable_Generic , Encodable , Decodable)] pub struct Ident { pub name : Symbol , pub span : Span , }