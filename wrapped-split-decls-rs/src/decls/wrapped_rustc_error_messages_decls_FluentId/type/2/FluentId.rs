use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Identifier for the Fluent message/attribute corresponding to a diagnostic message."] type FluentId = Cow < 'static , str > ;
}