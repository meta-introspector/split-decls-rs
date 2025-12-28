use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug)] pub struct Impl { pub generics : Generics , pub of_trait : Option < Box < TraitImplHeader > > , pub self_ty : Box < Ty > , pub items : ThinVec < Box < AssocItem > > , }