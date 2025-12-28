macro_rules! deps {
    () => {
        UndoLog!();
        SnapshotMap!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl < K , V , M , L > Rollback < UndoLog < K , V > > for SnapshotMap < K , V , M , L > where K : Eq + Hash , M : Rollback < UndoLog < K , V > > , { fn reverse (& mut self , undo : UndoLog < K , V >) { self . map . reverse (undo) } }
    };
}

impl_423!();