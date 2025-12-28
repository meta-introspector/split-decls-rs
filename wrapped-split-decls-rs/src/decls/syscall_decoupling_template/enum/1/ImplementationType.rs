use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub enum ImplementationType { Production , Mock , Logged , Governed , }