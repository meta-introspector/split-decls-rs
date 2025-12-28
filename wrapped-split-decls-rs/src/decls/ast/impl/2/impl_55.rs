use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl GenericParam { pub fn span (& self) -> Span { match & self . kind { GenericParamKind :: Lifetime | GenericParamKind :: Type { default : None } => { self . ident . span } GenericParamKind :: Type { default : Some (ty) } => self . ident . span . to (ty . span) , GenericParamKind :: Const { span , .. } => * span , } } }