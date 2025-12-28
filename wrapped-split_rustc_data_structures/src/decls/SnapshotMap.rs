macro_rules! deps {
    () => {
        UndoLog!();
    };
}

macro_rules! SnapshotMap {
    () => {
        deps!();
        # [derive (Clone)] pub struct SnapshotMap < K , V , M = FxHashMap < K , V > , L = VecLog < UndoLog < K , V > > > { map : M , undo_log : L , _marker : PhantomData < (K , V) > , }
    };
}

SnapshotMap!()