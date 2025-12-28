use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct ParamsPrinter < 'i > (& 'i [ChallengeParamRef < 'i >]) ;