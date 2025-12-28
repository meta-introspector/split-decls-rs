macro_rules! deps {
    () => {
        LRUCache!();
    };
}

macro_rules! items {
    () => {
        deps!();
        # [doc = " Convenience function for test assertions"] fn items < T , const N : usize > (cache : & mut LRUCache < T , N >) -> Vec < T > where T : Clone , { let mut v = Vec :: new () ; let mut iter = cache . iter_mut () ; while let Some ((_idx , val)) = iter . next () { v . push (val . clone ()) } v }
    };
}

items!()