use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An asynchronous [`Stream`] of arriving signals."] # [doc = ""] # [doc = " The stream doesn't return the signals in the order they were recieved by"] # [doc = " the process and may merge signals received multiple times."] pub struct SignalsInfo < E : Exfiltrator = SignalOnly > (OwningSignalIterator < UnixStream , E >) ;