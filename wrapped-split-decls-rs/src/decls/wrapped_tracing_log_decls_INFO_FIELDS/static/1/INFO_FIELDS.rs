use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static INFO_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& INFO_CS)) ;