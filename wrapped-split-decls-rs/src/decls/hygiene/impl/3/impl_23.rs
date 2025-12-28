use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SyntaxContextData { fn root () -> SyntaxContextData { SyntaxContextData { outer_expn : ExpnId :: root () , outer_transparency : Transparency :: Opaque , parent : SyntaxContext :: root () , opaque : SyntaxContext :: root () , opaque_and_semiopaque : SyntaxContext :: root () , dollar_crate_name : kw :: DollarCrate , } } fn key (& self) -> SyntaxContextKey { (self . parent , self . outer_expn , self . outer_transparency) } }