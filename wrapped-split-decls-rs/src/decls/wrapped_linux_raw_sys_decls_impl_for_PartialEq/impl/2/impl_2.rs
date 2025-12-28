use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "general")] impl PartialEq for general :: __kernel_timespec { fn eq (& self , other : & Self) -> bool { ({ let Self { tv_sec , tv_nsec } = self ; (tv_sec , tv_nsec) }) == ({ let Self { tv_sec , tv_nsec } = other ; (tv_sec , tv_nsec) }) } }