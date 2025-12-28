use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct TestOutcome < O , E > { pub completed : Vec < O > , pub errors : Vec < Error < O , E > > , }
}