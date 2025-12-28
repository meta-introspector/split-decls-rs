macro_rules! deps {
    () => {
        Map!();
        Downcast!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        # [doc = " A view into a single occupied location in an `Map`."] pub struct OccupiedEntry < 'map , A : ? Sized + Downcast , V : 'map > { inner : hash_map :: OccupiedEntry < 'map , TypeId , Box < A > > , type_ : PhantomData < V > , }
    };
}

OccupiedEntry!();