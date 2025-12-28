use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L , R > Deref for Either < L , R > where L : Deref , R : Deref < Target = L :: Target > , { type Target = L :: Target ; fn deref (& self) -> & Self :: Target { for_both ! (self , inner => &** inner) } }