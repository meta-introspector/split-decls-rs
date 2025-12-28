macro_rules! ThreadWaker {
    () => {
        # [derive (Debug)] struct ThreadWaker { state : Mutex < usize > , condvar : Condvar , }
    };
}

ThreadWaker!();