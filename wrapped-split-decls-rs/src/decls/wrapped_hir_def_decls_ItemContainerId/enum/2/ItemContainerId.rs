use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemContainerId { ExternBlockId (ExternBlockId) , ModuleId (ModuleId) , ImplId (ImplId) , TraitId (TraitId) , }