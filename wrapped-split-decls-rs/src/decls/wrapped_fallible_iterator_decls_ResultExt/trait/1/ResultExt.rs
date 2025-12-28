use serde::{Deserialize, Serialize};
use std::collections::HashMap;

trait ResultExt < T , E > { fn unpack_fold (self) -> Result < T , E > ; }