use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < std :: io :: Error > for GetTimezoneError { fn from (orig : std :: io :: Error) -> Self { GetTimezoneError :: IoError (orig) } }
}