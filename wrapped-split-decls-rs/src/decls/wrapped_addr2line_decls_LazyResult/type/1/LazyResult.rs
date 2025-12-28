use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type LazyResult < T > = OnceCell < Result < T , Error > > ;
}