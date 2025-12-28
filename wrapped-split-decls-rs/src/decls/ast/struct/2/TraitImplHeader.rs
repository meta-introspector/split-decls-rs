use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug)] pub struct TraitImplHeader { pub defaultness : Defaultness , pub safety : Safety , pub constness : Const , pub polarity : ImplPolarity , pub trait_ref : TraitRef , }