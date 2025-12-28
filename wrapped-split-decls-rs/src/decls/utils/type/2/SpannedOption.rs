use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An [`Option<T>`] that keeps track of the span that caused it to be set; used with [`SetOnce`]."] pub (super) type SpannedOption < T > = Option < (T , Span) > ;