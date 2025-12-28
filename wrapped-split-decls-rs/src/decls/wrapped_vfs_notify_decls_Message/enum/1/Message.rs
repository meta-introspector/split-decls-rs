use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] enum Message { Config (loader :: Config) , Invalidate (AbsPathBuf) , }
}