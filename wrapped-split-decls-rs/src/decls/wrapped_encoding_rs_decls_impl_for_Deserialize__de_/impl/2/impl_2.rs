use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for & 'static Encoding { fn deserialize < D > (deserializer : D) -> Result < & 'static Encoding , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (EncodingVisitor) } }