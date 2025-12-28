use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "std")] impl From < TryGetError > for std :: io :: Error { fn from (error : TryGetError) -> Self { std :: io :: Error :: new (std :: io :: ErrorKind :: Other , error) } }
}