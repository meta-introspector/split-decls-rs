macro_rules! ThreadBound {
    () => {
        # [doc = " ThreadBound is a Sync-maker and Send-maker that allows accessing a value"] # [doc = " of type T only from the original thread on which the ThreadBound was"] # [doc = " constructed."] pub (crate) struct ThreadBound < T > { value : T , thread_id : ThreadId , }
    };
}

ThreadBound!();