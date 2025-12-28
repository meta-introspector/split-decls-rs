macro_rules! deps {
    () => {
        Slab!();
        Entry!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " A mutable iterator over the values stored in the `Slab`"] pub struct IterMut < 'a , T > { entries : iter :: Enumerate < slice :: IterMut < 'a , Entry < T > > > , len : usize , }
    };
}

IterMut!()