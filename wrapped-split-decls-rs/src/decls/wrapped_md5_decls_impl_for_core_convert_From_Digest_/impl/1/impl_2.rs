use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl core :: convert :: From < Digest > for [u8 ; 16] { # [inline] fn from (digest : Digest) -> Self { digest . 0 } }