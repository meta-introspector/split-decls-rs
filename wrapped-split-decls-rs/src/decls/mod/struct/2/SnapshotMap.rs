use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone)] pub struct SnapshotMap < K , V , M = FxHashMap < K , V > , L = VecLog < UndoLog < K , V > > > { map : M , undo_log : L , _marker : PhantomData < (K , V) > , }