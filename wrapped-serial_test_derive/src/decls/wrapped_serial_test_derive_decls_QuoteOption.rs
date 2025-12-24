use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Default, Debug, Clone)]
struct QuoteOption<T>(Option<T>);
