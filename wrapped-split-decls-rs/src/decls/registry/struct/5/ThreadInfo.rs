use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ThreadInfo { # [doc = " Latch set once thread has started and we are entering into the"] # [doc = " main loop. Used to wait for worker threads to become primed,"] # [doc = " primarily of interest for benchmarking."] primed : LockLatch , # [doc = " Latch is set once worker thread has completed. Used to wait"] # [doc = " until workers have stopped; only used for tests."] stopped : LockLatch , # [doc = " The latch used to signal that terminated has been requested."] # [doc = " This latch is *set* by the `terminate` method on the"] # [doc = " `Registry`, once the registry's main \"terminate\" counter"] # [doc = " reaches zero."] terminate : OnceLatch , # [doc = " the \"stealer\" half of the worker's deque"] stealer : Stealer < JobRef > , }
}