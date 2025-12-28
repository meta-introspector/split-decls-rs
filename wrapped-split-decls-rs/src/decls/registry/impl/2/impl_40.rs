use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for WorkerThread { fn drop (& mut self) { WORKER_THREAD_STATE . with (| t | { assert ! (t . get () . eq (& (self as * const _))) ; t . set (ptr :: null ()) ; }) ; } }