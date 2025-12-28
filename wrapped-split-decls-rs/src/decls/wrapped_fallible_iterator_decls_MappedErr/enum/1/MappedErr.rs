use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum MappedErr < T , U > { It (T) , Fold (U) , }
}