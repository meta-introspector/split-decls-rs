macro_rules! deps {
    () => {
        UndoLog!();
        SnapshotMap!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl < K , V , M , L > SnapshotMap < K , V , M , L > where K : Hash + Clone + Eq , M : BorrowMut < FxHashMap < K , V > > + Borrow < FxHashMap < K , V > > , L : UndoLogs < UndoLog < K , V > > , { pub fn clear (& mut self) { self . map . borrow_mut () . clear () ; self . undo_log . clear () ; } pub fn insert (& mut self , key : K , value : V) -> bool { match self . map . borrow_mut () . insert (key . clone () , value) { None => { self . undo_log . push (UndoLog :: Inserted (key)) ; true } Some (old_value) => { self . undo_log . push (UndoLog :: Overwrite (key , old_value)) ; false } } } pub fn remove (& mut self , key : K) -> bool { match self . map . borrow_mut () . remove (& key) { Some (old_value) => { self . undo_log . push (UndoLog :: Overwrite (key , old_value)) ; true } None => false , } } pub fn get (& self , key : & K) -> Option < & V > { self . map . borrow () . get (key) } }
    };
}

impl_420!();