use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < hir_ty :: next_solver :: Mutability > for Access { fn from (mutability : hir_ty :: next_solver :: Mutability) -> Access { match mutability { hir_ty :: next_solver :: Mutability :: Not => Access :: Shared , hir_ty :: next_solver :: Mutability :: Mut => Access :: Exclusive , } } }