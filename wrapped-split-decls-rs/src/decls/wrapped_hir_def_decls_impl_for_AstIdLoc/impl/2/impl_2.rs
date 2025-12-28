use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < N : AstIdNode > AstIdLoc for AssocItemLoc < N > { type Container = ItemContainerId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }