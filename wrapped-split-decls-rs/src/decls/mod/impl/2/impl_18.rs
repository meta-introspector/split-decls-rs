use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , V > Rollback < UndoLog < K , V > > for FxHashMap < K , V > where K : Eq + Hash , { fn reverse (& mut self , undo : UndoLog < K , V >) { match undo { UndoLog :: Inserted (key) => { self . remove (& key) ; } UndoLog :: Overwrite (key , old_value) => { self . insert (key , old_value) ; } UndoLog :: Purged => { } } } }