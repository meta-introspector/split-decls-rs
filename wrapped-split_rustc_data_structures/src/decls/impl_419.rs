macro_rules! deps {
    () => {
        SnapshotMap!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl < K , V , M , L > SnapshotMap < K , V , M , L > { # [inline] pub fn with_log < L2 > (& mut self , undo_log : L2) -> SnapshotMap < K , V , & mut M , L2 > { SnapshotMap { map : & mut self . map , undo_log , _marker : PhantomData } } }
    };
}

impl_419!();