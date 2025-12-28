use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Associates each `Expectation` with its place in a [`Sequence`]."] # [doc (hidden)] pub struct SeqHandle { inner : Arc < SeqInner > , # [doc = " An ID counter for every `SeqHandle` associated with the same"] # [doc = " [`Sequence`].  Starts at 0 and counts upwards."] seq : usize , }