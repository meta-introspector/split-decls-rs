use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Indirect queue to provide FIFO job priority."] pub (super) struct JobFifo { inner : Injector < JobRef > , }