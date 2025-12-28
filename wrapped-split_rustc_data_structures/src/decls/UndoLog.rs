macro_rules! UndoLog {
    () => {
        # [derive (Clone)] pub enum UndoLog < K , V > { Inserted (K) , Overwrite (K , V) , Purged , }
    };
}

UndoLog!()