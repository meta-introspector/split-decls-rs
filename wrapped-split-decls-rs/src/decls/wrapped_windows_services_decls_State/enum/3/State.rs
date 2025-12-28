use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Possible service states including transitional states."] # [doc = ""] # [doc = " This can be useful to query the current state with the `state` function or set the state with"] # [doc = " the `set_state` function."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum State { ContinuePending , Paused , PausePending , Running , StartPending , Stopped , StopPending , }
}