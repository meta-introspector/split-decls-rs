use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents a list of threads which can access worker locals."] # [derive (Clone)] pub struct Registry (Arc < RegistryData >) ;