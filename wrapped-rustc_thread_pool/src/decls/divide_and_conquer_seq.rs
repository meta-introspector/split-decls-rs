macro_rules! divide_and_conquer_seq {
    () => {
        fn divide_and_conquer_seq (counter : & AtomicUsize , size : usize) { if size > 1 { divide_and_conquer_seq (counter , size / 2) ; divide_and_conquer_seq (counter , size / 2) ; } else { counter . fetch_add (1 , Ordering :: SeqCst) ; } }
    };
}

divide_and_conquer_seq!();