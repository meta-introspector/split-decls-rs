macro_rules! deps {
    () => {
        Slab!();
        Entry!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over the values stored in the `Slab`"] pub struct Iter < 'a , T > { entries : iter :: Enumerate < slice :: Iter < 'a , Entry < T > > > , len : usize , }
    };
}

Iter!()