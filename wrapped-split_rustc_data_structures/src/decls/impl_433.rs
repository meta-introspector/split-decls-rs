macro_rules! deps {
    () => {
        SortedMap!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl < 'a , K , Q , V > IndexMut < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { fn index_mut (& mut self , key : & Q) -> & mut Self :: Output { self . get_mut (key) . expect ("no entry found for key") } }
    };
}

impl_433!()