use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Ord for Value {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (&Value::Bool(v0), &Value::Bool(ref v1)) => v0.cmp(v1),
            (&Value::U8(v0), &Value::U8(ref v1)) => v0.cmp(v1),
            (&Value::U16(v0), &Value::U16(ref v1)) => v0.cmp(v1),
            (&Value::U32(v0), &Value::U32(ref v1)) => v0.cmp(v1),
            (&Value::U64(v0), &Value::U64(ref v1)) => v0.cmp(v1),
            (&Value::I8(v0), &Value::I8(ref v1)) => v0.cmp(v1),
            (&Value::I16(v0), &Value::I16(ref v1)) => v0.cmp(v1),
            (&Value::I32(v0), &Value::I32(ref v1)) => v0.cmp(v1),
            (&Value::I64(v0), &Value::I64(ref v1)) => v0.cmp(v1),
            (&Value::F32(v0), &Value::F32(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Value::F64(v0), &Value::F64(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Value::Char(v0), &Value::Char(ref v1)) => v0.cmp(v1),
            (&Value::String(ref v0), &Value::String(ref v1)) => v0.cmp(v1),
            (&Value::Unit, &Value::Unit) => Ordering::Equal,
            (&Value::Option(ref v0), &Value::Option(ref v1)) => v0.cmp(v1),
            (&Value::Newtype(ref v0), &Value::Newtype(ref v1)) => v0.cmp(v1),
            (&Value::Seq(ref v0), &Value::Seq(ref v1)) => v0.cmp(v1),
            (&Value::Map(ref v0), &Value::Map(ref v1)) => v0.cmp(v1),
            (&Value::Bytes(ref v0), &Value::Bytes(ref v1)) => v0.cmp(v1),
            (ref v0, ref v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}
