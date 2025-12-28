macro_rules! Coordinate {
    () => {
        # [doc = " A simplified `WaitGroup`, this is used together with `Arc<Zalsa>` as the actual counter"] struct Coordinate { coordinate_lock : Mutex < () > , cvar : Condvar , }
    };
}

Coordinate!()