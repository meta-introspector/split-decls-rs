use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl LocalKind { pub fn init (& self) -> Option < & Expr > { match self { Self :: Decl => None , Self :: Init (i) | Self :: InitElse (i , _) => Some (i) , } } pub fn init_else_opt (& self) -> Option < (& Expr , Option < & Block >) > { match self { Self :: Decl => None , Self :: Init (init) => Some ((init , None)) , Self :: InitElse (init , els) => Some ((init , Some (els))) , } } }