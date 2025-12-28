use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , V , M , L > Rollback < UndoLog < K , V > > for SnapshotMap < K , V , M , L > where K : Eq + Hash , M : Rollback < UndoLog < K , V > > , { fn reverse (& mut self , undo : UndoLog < K , V >) { self . map . reverse (undo) } }