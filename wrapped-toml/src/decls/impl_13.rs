macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [doc = " Mutably access an element of this map. Panics if the given key is not"] # [doc = " present in the map."] impl < K , V , Q > ops :: IndexMut < & Q > for Map < K , V > where K : Borrow < Q > + Ord , Q : Ord + Eq + Hash + ? Sized , { fn index_mut (& mut self , index : & Q) -> & mut V { self . map . get_mut (index) . expect ("no entry found for key") } }
    };
}

impl_13!()