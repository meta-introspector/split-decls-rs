use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < Ctx : Copy > SpanData < Ctx > { pub fn eq_ignoring_ctx (self , other : Self) -> bool { self . anchor == other . anchor && self . range == other . range } }