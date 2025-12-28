use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ImplCallVisitor { calls : std :: collections :: HashMap < String , std :: collections :: HashSet < String > > , }
}