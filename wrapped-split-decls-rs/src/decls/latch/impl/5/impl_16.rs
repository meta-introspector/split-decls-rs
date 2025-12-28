use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'r > SpinLatch < 'r > { # [doc = " Creates a new spin latch that is owned by `thread`. This means"] # [doc = " that `thread` is the only thread that should be blocking on"] # [doc = " this latch -- it also means that when the latch is set, we"] # [doc = " will wake `thread` if it is sleeping."] # [inline] pub (super) fn new (thread : & 'r WorkerThread) -> SpinLatch < 'r > { SpinLatch { core_latch : CoreLatch :: new () , registry : thread . registry () , target_worker_index : thread . index () , cross : false , } } # [doc = " Creates a new spin latch for cross-threadpool blocking. Notably, we"] # [doc = " need to make sure the registry is kept alive after setting, so we can"] # [doc = " safely call the notification."] # [inline] pub (super) fn cross (thread : & 'r WorkerThread) -> SpinLatch < 'r > { SpinLatch { cross : true , .. SpinLatch :: new (thread) } } }
}