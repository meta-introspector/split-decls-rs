use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : HasTokens > HasTokens for Box < T > { fn tokens (& self) -> Option < & LazyAttrTokenStream > { (* * self) . tokens () } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { (* * self) . tokens_mut () } }
}