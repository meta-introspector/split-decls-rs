use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static TRACE_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& TRACE_CS)) ;