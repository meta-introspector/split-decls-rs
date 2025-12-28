macro_rules! take_array {
    () => {
        fn take_array < T , const N : usize > (b : & mut Box < [T] >) -> Result < [T ; N] , Box < [T] > > { let b : Box < [T ; N] > = std :: mem :: take (b) . try_into () ? ; Ok (* b) }
    };
}

take_array!()