use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum FoldStop < T , E > { Break (T) , Err (E) , }
}