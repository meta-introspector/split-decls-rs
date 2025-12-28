macro_rules! deps {
    () => {
        Entry!();
        Slab!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " A draining iterator for `Slab`"] pub struct Drain < 'a , T > { inner : vec :: Drain < 'a , Entry < T > > , len : usize , }
    };
}

Drain!();