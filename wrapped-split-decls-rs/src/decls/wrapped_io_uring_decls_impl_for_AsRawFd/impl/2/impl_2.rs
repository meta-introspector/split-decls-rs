use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > AsRawFd for IoUring < S , C > { fn as_raw_fd (& self) -> RawFd { self . fd . as_raw_fd () } }