use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An instance of this struct is created when a thread becomes idle."] # [doc = " It is consumed when the thread finds work, and passed by `&mut`"] # [doc = " reference for operations that preserve the idle state. (In other"] # [doc = " words, producing one of these structs is evidence the thread is"] # [doc = " idle.) It tracks state such as how long the thread has been idle."] pub (super) struct IdleState { # [doc = " What is worker index of the idle thread?"] worker_index : usize , # [doc = " How many rounds have we been circling without sleeping?"] rounds : u32 , # [doc = " Once we become sleepy, what was the sleepy counter value?"] # [doc = " Set to `INVALID_SLEEPY_COUNTER` otherwise."] jobs_counter : JobsEventCounter , }
}