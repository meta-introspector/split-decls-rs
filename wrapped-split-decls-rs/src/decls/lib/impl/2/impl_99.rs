use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for SpanData { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . span () , f) } }