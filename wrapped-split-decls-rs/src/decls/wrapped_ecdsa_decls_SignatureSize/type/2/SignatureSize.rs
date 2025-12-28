use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Size of a fixed sized signature for the given elliptic curve."] pub type SignatureSize < C > = < FieldBytesSize < C > as Add > :: Output ;
}