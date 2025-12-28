use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Identifier for the Fluent message/attribute corresponding to a diagnostic message."] type FluentId = Cow < 'static , str > ;