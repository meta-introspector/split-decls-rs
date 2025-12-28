use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ParamsPrinter < 'i > (& 'i [ChallengeParamRef < 'i >]) ;
}