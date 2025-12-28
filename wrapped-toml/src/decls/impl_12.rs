macro_rules! deps {
    () => {
        Map!();
        Index!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [doc = " Access an element of this map. Panics if the given key is not present in the"] # [doc = " map."] impl < K , V , Q > ops :: Index < & Q > for Map < K , V > where K : Borrow < Q > + Ord , Q : Ord + Eq + Hash + ? Sized , { type Output = V ; fn index (& self , index : & Q) -> & V { self . map . index (index) } }
    };
}

impl_12!();