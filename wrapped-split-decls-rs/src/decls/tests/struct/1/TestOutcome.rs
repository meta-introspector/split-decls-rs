use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct TestOutcome < O , E > { pub completed : Vec < O > , pub errors : Vec < Error < O , E > > , }