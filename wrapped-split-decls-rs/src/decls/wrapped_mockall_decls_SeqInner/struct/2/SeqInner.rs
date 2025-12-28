use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Default)] struct SeqInner { # [doc = " Should match the `seq` field of the next [`SeqHandle`] that has not been"] # [doc = " fully satisfied."] satisfaction_level : AtomicUsize , }