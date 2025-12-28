use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Tlv { # [inline] pub (crate) fn null () -> Self { Self (ptr :: null ()) } }