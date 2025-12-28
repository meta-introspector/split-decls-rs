macro_rules! count_handler {
    () => {
        # [doc = " Creates a start/exit handler that increments an atomic counter."] fn count_handler () -> (Arc < AtomicUsize > , impl Fn (usize)) { let count = Arc :: new (AtomicUsize :: new (0)) ; (Arc :: clone (& count) , move | _ | { count . fetch_add (1 , Ordering :: SeqCst) ; }) }
    };
}

count_handler!();