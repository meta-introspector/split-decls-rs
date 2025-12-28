macro_rules! AssertDropped {
    () => {
        struct AssertDropped { dropped : std :: sync :: Arc < AtomicBool > , }
    };
}

AssertDropped!()