use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A trait for AST nodes having (or not having) collected tokens."] pub trait HasTokens { fn tokens (& self) -> Option < & LazyAttrTokenStream > ; fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > ; }