use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Buffer for block processing of data."] pub struct BlockBuffer < BS : ArraySize , K : BufferKind > { buffer : MaybeUninit < Array < u8 , BS > > , pos : K :: Pos , }