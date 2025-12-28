macro_rules! State {
    () => {
        # [derive (Default , Debug)] struct State { is_dropped : AtomicBool , is_cleared : AtomicBool , id : usize , }
    };
}

State!()