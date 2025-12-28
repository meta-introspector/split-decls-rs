use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type ChallengeParamRef < 'i > = (& 'i str , ParamValue < 'i >) ;
}