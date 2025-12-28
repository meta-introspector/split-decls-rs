macro_rules! deps {
    () => {
        Downcast!();
        Map!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A view into a single empty location in an `Map`."] pub struct VacantEntry < 'map , A : ? Sized + Downcast , V : 'map > { inner : hash_map :: VacantEntry < 'map , TypeId , Box < A > > , type_ : PhantomData < V > , }
    };
}

VacantEntry!()