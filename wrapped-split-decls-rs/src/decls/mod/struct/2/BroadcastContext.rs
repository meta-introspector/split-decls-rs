use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Provides context to a closure called by `broadcast`."] pub struct BroadcastContext < 'a > { worker : & 'a WorkerThread , # [doc = " Make sure to prevent auto-traits like `Send` and `Sync`."] _marker : PhantomData < & 'a mut dyn Fn () > , }
}