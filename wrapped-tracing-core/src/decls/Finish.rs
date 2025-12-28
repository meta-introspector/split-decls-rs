macro_rules! Finish {
    () => {
        struct Finish < 'a > { state : & 'a AtomicUsize , panicked : bool , }
    };
}

Finish!()