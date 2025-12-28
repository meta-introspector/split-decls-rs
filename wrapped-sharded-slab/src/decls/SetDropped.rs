macro_rules! SetDropped {
    () => {
        struct SetDropped { val : usize , dropped : std :: sync :: Arc < AtomicBool > , }
    };
}

SetDropped!()