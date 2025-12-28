use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , V , HCX > ! HashStable < HCX > for std :: collections :: HashMap < K , V > { }
}