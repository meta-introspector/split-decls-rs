use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ConfigTrait for MockConfig { fn load () -> Self { MockConfig { rule : vec ! [Rule { kind : RuleKind :: AddDerive , trait_name : vec ! ["Debug" . to_string () , "PartialEq" . to_string ()] , apply_to : vec ! [] , condition : "true" . to_string () , }] , } } fn get_rules (& self) -> & Vec < Rule > { & self . rule } }