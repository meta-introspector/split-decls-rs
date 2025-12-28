macro_rules! deps {
    () => {
        SnapshotMap!();
        Index!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl < 'k , K , V , M , L > ops :: Index < & 'k K > for SnapshotMap < K , V , M , L > where K : Hash + Clone + Eq , M : Borrow < FxHashMap < K , V > > , { type Output = V ; fn index (& self , key : & 'k K) -> & V { & self . map . borrow () [key] } }
    };
}

impl_422!();