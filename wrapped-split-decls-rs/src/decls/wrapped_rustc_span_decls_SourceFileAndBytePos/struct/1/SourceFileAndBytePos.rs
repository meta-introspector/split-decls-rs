use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct SourceFileAndBytePos { pub sf : Arc < SourceFile > , pub pos : BytePos , }