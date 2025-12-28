use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < AssocItemId > for AttrDefId { fn from (assoc : AssocItemId) -> Self { match assoc { AssocItemId :: FunctionId (it) => AttrDefId :: FunctionId (it) , AssocItemId :: ConstId (it) => AttrDefId :: ConstId (it) , AssocItemId :: TypeAliasId (it) => AttrDefId :: TypeAliasId (it) , } } }