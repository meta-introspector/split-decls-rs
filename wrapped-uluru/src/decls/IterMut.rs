macro_rules! deps {
    () => {
        LRUCache!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " Mutable iterator over values in an `LRUCache`, from most-recently-used to least-recently-used."] struct IterMut < 'a , T , const N : usize > { cache : & 'a mut LRUCache < T , N > , pos : u16 , }
    };
}

IterMut!()