use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait PpAnn { fn nested (& self , _state : & mut State < '_ > , _nested : Nested) { } fn pre (& self , _state : & mut State < '_ > , _node : AnnNode < '_ >) { } fn post (& self , _state : & mut State < '_ > , _node : AnnNode < '_ >) { } }
}