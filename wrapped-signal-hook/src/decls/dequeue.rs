macro_rules! dequeue {
    () => {
        fn dequeue (q : & AtomicU16) -> Option < u16 > { let mut current = q . load (Ordering :: Relaxed) ; loop { let val = current & MASK ; if val == 0 { break None ; } let modified = current >> BITS ; match q . compare_exchange_weak (current , modified , Ordering :: Acquire , Ordering :: Relaxed) { Ok (_) => break Some (val) , Err (changed) => current = changed , } } }
    };
}

dequeue!()